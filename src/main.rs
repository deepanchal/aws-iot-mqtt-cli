mod format;

use crate::format::format_mqtt_log_entry;
use aws_iot_device_sdk_rust::settings::MQTTOptionsOverrides;
use aws_iot_device_sdk_rust::{
    AWSIoTAsyncClient, AWSIoTSettings, Packet, QoS, async_event_loop_listener,
};
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use log::debug;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{Duration, sleep};

/// MQTT CLI for AWS IoT
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = r#"
MQTT CLI for AWS IoT

This tool allows you to subscribe to or publish messages to AWS IoT topics.
You can filter messages from topics using regex patterns for inclusion or exclusion.

Examples:
  aws-iot-mqtt-cli sub --topics test/1234/health,test/2345/data
  aws-iot-mqtt-cli sub --topics test/1234/health,test/2345/state --include "1234"
  aws-iot-mqtt-cli pub --topics test/1234/health,test/2345/state --message '{"data": 123}'
"#
)]
struct Args {
    #[command(subcommand)]
    command: Option<CliCommand>,

    /// AWS IoT endpoint URL
    #[arg(long, env = "AWS_IOT_ENDPOINT")]
    endpoint: String,

    /// AWS IoT endpoint URL
    #[arg(long, env = "AWS_IOT_PORT", default_value = "8883")]
    port: u16,

    /// Client ID for MQTT connection
    #[arg(long, env = "AWS_IOT_CLIENT_ID")]
    client_id: String,

    /// Path to the root CA certificate
    #[arg(
        long,
        env = "AWS_IOT_ROOT_CA_PATH",
        default_value = "./certs/AmazonRootCA1.pem"
    )]
    root_ca_path: String,

    /// Path to the device certificate
    #[arg(
        long,
        env = "AWS_IOT_DEVICE_CERT_PATH",
        default_value = "./certs/cert.crt"
    )]
    device_cert_path: String,

    /// Path to the device private key
    #[arg(
        long,
        env = "AWS_IOT_DEVICE_PRIVATE_KEY_PATH",
        default_value = "./certs/key.pem"
    )]
    device_private_key_path: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
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

fn setup_logging(verbose: bool) {
    if verbose {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut cmd = Args::command();

    setup_logging(args.verbose);
    debug!("Parsed CLI arguments: {:?}", args);

    let endpoint = args.endpoint;
    let port = args.port;
    let client_id = args.client_id;
    let root_ca_path = args.root_ca_path;
    let device_cert_path = args.device_cert_path;
    let device_private_key_path = args.device_private_key_path;
    let mqtt_option_overrides = MQTTOptionsOverrides {
        port: Some(port),
        clean_session: Some(true),
        keep_alive: None,
        max_packet_size: None,
        request_channel_capacity: None,
        pending_throttle: None,
        inflight: None,
        last_will: None,
        conn_timeout: None,
        transport: None,
    };
    let aws_settings = AWSIoTSettings::new(
        client_id.clone(),
        root_ca_path.clone(),
        device_cert_path.clone(),
        device_private_key_path.clone(),
        endpoint.clone(),
        Some(mqtt_option_overrides),
    );

    debug!(
        "Connecting to {} with client_id: {}",
        endpoint.clone().blue(),
        client_id.clone().blue(),
    );

    let (iot_core_client, (event_loop, sender)) = AWSIoTAsyncClient::new(aws_settings).await?;
    let raw_client = iot_core_client.get_client().await;
    let client = Arc::new(Mutex::new(raw_client));

    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        debug!("Received shutdown signal, cleaning up...");
    };

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

            for topic in topic_list.clone() {
                client
                    .lock()
                    .await
                    .subscribe(topic.to_string(), QoS::AtMostOnce)
                    .await?;
                println!("{}", format!("Subscribed to topic: {}", topic).blue());
            }

            // For subscriptions, keep listening to messages
            let receiver = sender.subscribe();
            let receiver = Arc::new(Mutex::new(receiver));

            let recv_thread = task::spawn(async move {
                loop {
                    if let Ok(Packet::Publish(p)) = receiver.lock().await.recv().await {
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
                        let formatted_output = format_mqtt_log_entry(&topic, &payload);
                        println!("{}", formatted_output);
                    }
                }
            });

            let listen_thread = task::spawn(async move {
                async_event_loop_listener((event_loop, sender))
                    .await
                    .unwrap();
            });

            // Wait for either the threads to complete or the shutdown signal
            tokio::select! {
                _ = recv_thread => {}
                _ = listen_thread => {}
                _ = shutdown_signal => {
                    for topic in topic_list.clone() {
                        client.lock().await.unsubscribe(topic).await?;
                        println!("{}", format!("Unsubscribed from topic: {}", topic).blue());
                    }
                }
            }
        }

        Some(CliCommand::Pub { topics, message }) => {
            let topic_list: Vec<&str> = topics.split(',').collect();

            // Create a receiver to drain incoming events
            let receiver = sender.subscribe();
            let receiver = Arc::new(Mutex::new(receiver));
            let drain_task = task::spawn(async move {
                while (receiver.lock().await.recv().await).is_ok() {
                    match receiver.lock().await.recv().await {
                        Ok(_) => {}      // Ignore incoming events
                        Err(_) => break, // Exit if the channel is closed
                    }
                }
            });

            for topic in topic_list {
                client
                    .lock()
                    .await
                    .publish(topic, QoS::AtMostOnce, false, message.clone())
                    .await?;
                println!("{}", format!("Published to topic: {}", topic).blue());
                match serde_json::from_str::<Value>(&message) {
                    Ok(parsed) => println!("{}", serde_json::to_string_pretty(&parsed)?.white()),
                    Err(_) => println!("{}", message.white()), // Fallback to raw message
                };
            }

            // Run the event loop briefly to ensure the message is sent
            let event_loop_task = task::spawn(async move {
                async_event_loop_listener((event_loop, sender))
                    .await
                    .unwrap();
            });

            // Allow time for the event loop to process the message
            sleep(Duration::from_secs(1)).await;
            event_loop_task.abort();
            drain_task.abort();
        }

        None => cmd.print_long_help()?,
    }

    Ok(())
}
