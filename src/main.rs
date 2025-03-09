use aws_iot_device_sdk_rust::{
    async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings, Packet, QoS,
};
use clap::{Parser, Subcommand};
use colored::*;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};

/// MQTT CLI for AWS IoT
#[derive(Parser, Debug)]
#[command(
    version,
    arg_required_else_help = true,
    about,
    long_about = r#"
MQTT CLI for AWS IoT

This tool allows you to subscribe to or publish messages to AWS IoT topics.
You can filter messages using regex patterns for inclusion or exclusion.

Examples:
  aws-iot-mqtt-cli sub --topics test/1234/health,test/2345/data
  aws-iot-mqtt-cli sub --topics test/1234/health,test/2345/state --include "1234"
  aws-iot-mqtt-cli pub --topics test/1234/health,test/2345/state --message '{"data": 123}'
"#
)]
struct Args {
    #[command(subcommand)]
    command: Option<CliCommand>,

    /// Path to the root CA certificate
    #[arg(long, default_value = "./certs/AmazonRootCA1.pem")]
    root_ca: PathBuf,

    /// Path to the device certificate
    #[arg(long, default_value = "./certs/cert.crt")]
    device_cert: PathBuf,

    /// Path to the device private key
    #[arg(long, default_value = "./certs/key.pem")]
    private_key: PathBuf,
}

/// Subcommands for the CLI
#[derive(Subcommand, Debug)]
enum CliCommand {
    /// Subscribe to topics
    Sub {
        /// Comma-separated list of topics
        #[arg(short, long, default_value = "#")]
        topics: String,

        /// Regex to include topics
        #[arg(short, long)]
        include: Option<String>,

        /// Regex to exclude topics
        #[arg(short, long)]
        exclude: Option<String>,
    },

    /// Publish messages to topics
    Pub {
        /// Comma-separated list of topics
        #[arg(short, long)]
        topics: String,

        /// Message to publish
        #[arg(short, long)]
        message: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let endpoint = std::env::var("AWS_IOT_ENDPOINT").expect("AWS_IOT_ENDPOINT not set");
    let client_id = std::env::var("AWS_IOT_CLIENT_ID").expect("AWS_IOT_CLIENT_ID not set");
    let root_ca_path = args.root_ca.to_str().unwrap().to_string();
    let device_cert_path = args.device_cert.to_str().unwrap().to_string();
    let private_key_path = args.private_key.to_str().unwrap().to_string();
    println!(
        "{}",
        format!("Connecting with client_id: {}", client_id).blue()
    );

    let aws_settings = AWSIoTSettings::new(
        client_id,
        root_ca_path,
        device_cert_path,
        private_key_path,
        endpoint,
        None,
    );

    let (iot_core_client, event_loop) = AWSIoTAsyncClient::new(aws_settings).await?;
    let iot_core_client = Arc::new(Mutex::new(iot_core_client));

    match args.command {
        Some(CliCommand::Sub {
            topics,
            include,
            exclude,
        }) => {
            let topic_list: Vec<&str> = topics.split(',').collect();
            if let Some(regex_string) = include.clone() {
                println!(
                    "{}",
                    format!("Using include filter: {}", regex_string.red()).blue(),
                );
            }
            if let Some(regex_string) = exclude.clone() {
                println!(
                    "{}",
                    format!("Using exclude filter: {}", regex_string.red()).blue(),
                );
            }
            let include_regex = include.map(|s| Regex::new(&s).unwrap());
            let exclude_regex = exclude.map(|s| Regex::new(&s).unwrap());

            for topic in topic_list {
                iot_core_client
                    .lock()
                    .await
                    .subscribe(topic.to_string(), QoS::AtMostOnce)
                    .await?;
                println!("{}", format!("Subscribed to topic: {}", topic).blue());
            }

            // For subscriptions, keep listening to messages
            let receiver = iot_core_client.lock().await.get_receiver().await;
            let receiver = Arc::new(Mutex::new(receiver));

            let recv_thread = task::spawn(async move {
                loop {
                    match receiver.lock().await.recv().await {
                        Ok(Packet::Publish(p)) => {
                            let topic = p.topic;
                            let payload = match String::from_utf8(p.payload.to_vec()) {
                                Ok(s) => s,
                                Err(e) => {
                                    eprintln!("Invalid UTF-8 payload: {}", e);
                                    continue;
                                }
                            };
                            if let Some(ref regex) = include_regex {
                                if !regex.is_match(&topic) {
                                    continue;
                                }
                            }
                            if let Some(ref regex) = exclude_regex {
                                if regex.is_match(&topic) {
                                    continue;
                                }
                            }
                            format_mqtt_log_entry(&topic, &payload);
                        }
                        _ => (),
                    }
                }
            });

            let listen_thread = task::spawn(async move {
                async_event_loop_listener(event_loop).await.unwrap();
            });

            let (recv_result, listen_result) = tokio::join!(recv_thread, listen_thread);

            // Propagate errors if any
            recv_result?;
            listen_result?;
        }

        Some(CliCommand::Pub { topics, message }) => {
            let topic_list: Vec<&str> = topics.split(',').collect();

            // Create a receiver to drain incoming events
            let receiver = iot_core_client.lock().await.get_receiver().await;
            let receiver = Arc::new(Mutex::new(receiver));
            let drain_task = task::spawn(async move {
                loop {
                    match receiver.lock().await.recv().await {
                        Ok(_) => {}      // Ignore incoming events
                        Err(_) => break, // Exit if the channel is closed
                    }
                }
            });

            for topic in topic_list {
                iot_core_client
                    .lock()
                    .await
                    .publish(topic.to_string(), QoS::AtMostOnce, message.to_string())
                    .await?;
                println!("{}", format!("Published to topic: {}", topic).blue());
                match serde_json::from_str::<Value>(&message) {
                    Ok(parsed) => println!("{}", serde_json::to_string_pretty(&parsed)?.white()),
                    Err(_) => println!("{}", message.white()), // Fallback to raw message
                };
            }

            // Run the event loop briefly to ensure the message is sent
            let event_loop_task = task::spawn(async move {
                async_event_loop_listener(event_loop).await.unwrap();
            });

            // Allow time for the event loop to process the message
            sleep(Duration::from_secs(1)).await;
            event_loop_task.abort();
            drain_task.abort();
        }

        None => todo!(),
    }

    Ok(())
}

fn format_mqtt_log_entry(topic: &str, payload: &str) {
    let color = derive_color_from_string(topic);
    let timestamp = chrono::Utc::now().to_rfc3339();
    let pretty_output = match serde_json::from_str::<Value>(payload) {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| payload.to_string()),
        Err(_) => payload.to_string(),
    };

    let terminal_width = term_size::dimensions()
        .map(|(width, _)| width)
        .unwrap_or(96);

    let timestamp_width = timestamp.len();
    let max_topic_width = terminal_width.saturating_sub(timestamp_width + 1);

    let truncated_topic = if topic.len() > max_topic_width {
        format!("{}…", &topic[..max_topic_width.saturating_sub(1)])
    } else {
        topic.to_string()
    };

    let header_text = if topic.len() <= max_topic_width {
        let spacer_width = terminal_width.saturating_sub(truncated_topic.len() + timestamp_width);
        let spacer = " ".repeat(spacer_width);
        format!("{}{}{}", truncated_topic, spacer, timestamp)
    } else if topic.len() <= terminal_width {
        truncated_topic
    } else {
        format!("{}…", &topic[..terminal_width.saturating_sub(1)])
    };

    let divider = "─".repeat(header_text.len());
    let styled_divider = divider.color(color).bold();
    let styled_header = header_text.color(color).bold();
    let styled_json = pretty_output.bright_white();

    print_log_section(&styled_divider, &styled_header);
    println!("{}", styled_json);
    print_log_section(&styled_divider, &styled_header);
    println!();
}

/// Helper function to print header/footer sections
fn print_log_section(divider: &ColoredString, header: &ColoredString) {
    println!("{}", divider);
    println!("{}", header);
    println!("{}", divider);
}

fn derive_color_from_string(topic: &str) -> Color {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    topic.hash(&mut hasher);
    let hash = hasher.finish();

    // Generate vibrant colors using golden ratio distribution
    let hue = (hash as f64) * 0.618033988749895 % 360.0;
    let saturation = 75.0 + ((hash >> 8) % 25) as f64; // 75-100%
    let lightness = 45.0 + ((hash >> 16) % 15) as f64; // 45-60%

    let (r, g, b) = hsl_to_rgb(hue, saturation / 100.0, lightness / 100.0);
    Color::TrueColor { r, g, b }
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}
