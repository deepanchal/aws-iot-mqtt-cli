use colored::*;
use json_pretty_compact::PrettyCompactFormatter;
use serde_json::{Serializer, Value};
use serde::ser::Serialize;
use std::hash::{Hash, Hasher};

pub fn format_mqtt_log_entry(topic: &str, payload: &str) -> String {
    let color = derive_color_from_string(topic);
    let timestamp = chrono::Utc::now().to_rfc3339();
    let pretty_output = format_payload(payload);

    let (header_text, divider) = format_header_and_divider(topic, &timestamp, color);

    let log_section = print_log_section(&divider, &header_text);
    let formatted_output = format!(
        "{}\n{}\n{}\n",
        log_section,
        pretty_output.bright_white(),
        log_section,
    );

    formatted_output
}

fn format_payload(payload: &str) -> String {
    match serde_json::from_str::<Value>(payload) {
        Ok(value) => {
            let mut buffer = Vec::new();
            let formatter = PrettyCompactFormatter::new();
            let mut ser = Serializer::with_formatter(&mut buffer, formatter);
            match value.serialize(&mut ser) {
                Ok(_) => String::from_utf8_lossy(&buffer).into_owned(),
                Err(_) => payload.to_string(),
            }
        }
        Err(_) => payload.to_string(),
    }
}

fn format_header_and_divider(
    topic: &str,
    timestamp: &str,
    color: Color,
) -> (ColoredString, ColoredString) {
    let terminal_width = get_terminal_width().unwrap_or(96);
    let timestamp_width = timestamp.len();
    let max_topic_width = terminal_width.saturating_sub(timestamp_width + 1);

    let truncated_topic = truncate_topic(topic, max_topic_width);
    let header_text = format_header(&truncated_topic, timestamp, terminal_width, timestamp_width);
    let divider = "─".repeat(header_text.len());

    let styled_header = header_text.color(color).bold();
    let styled_divider = divider.color(color).bold();

    (styled_header, styled_divider)
}

fn truncate_topic(topic: &str, max_topic_width: usize) -> String {
    if topic.len() > max_topic_width {
        format!("{}…", &topic[..max_topic_width.saturating_sub(1)])
    } else {
        topic.to_string()
    }
}

fn format_header(
    truncated_topic: &str,
    timestamp: &str,
    terminal_width: usize,
    timestamp_width: usize,
) -> String {
    if truncated_topic.len() <= terminal_width {
        let spacer_width = terminal_width.saturating_sub(truncated_topic.len() + timestamp_width);
        let spacer = " ".repeat(spacer_width);
        format!("{}{}{}", truncated_topic, spacer, timestamp)
    } else {
        truncated_topic.to_string()
    }
}

fn get_terminal_width() -> Option<usize> {
    term_size::dimensions().map(|(width, _)| width)
}

fn print_log_section(divider: &ColoredString, header: &ColoredString) -> String {
    format!("{}\n{}\n{}", divider, header, divider)
}

fn derive_color_from_string(input: &str) -> Color {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input.hash(&mut hasher);
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
