use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use serde_json::Value;
use chrono::{DateTime, Utc, TimeZone, Datelike, LocalResult};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <filepath> <conversationId>", args[0]);
        std::process::exit(1);
    }
    let filepath = &args[1];
    let conversation_id = &args[2];
    let conversation_id = conversation_id.replace('\u{2068}', "");
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    let mut messages: Vec<(DateTime<Utc>, String)> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(json) = serde_json::from_str::<Value>(&line) {
                let conversation_matches = json.get("conversationId")
                    .and_then(|v| v.as_str())
                    .map_or(false, |s| s == conversation_id);

                if conversation_matches {
                    if let Some(sent_at) = json.get("sent_at").and_then(|v| v.as_i64()) {
                        if let Some(message_type) = json.get("type").and_then(|v| v.as_str()) {
                            match Utc.timestamp_opt(sent_at / 1000, 0) {
                                LocalResult::Single(datetime) => {
                                    messages.push((datetime, message_type.to_string()));
                                }
                                LocalResult::None => {
                                    eprintln!("Invalid timestamp: {}", sent_at);
                                }
                                LocalResult::Ambiguous(_, _) => {
                                    eprintln!("Ambiguous timestamp: {}", sent_at);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut messages_by_year: HashMap<i32, usize> = HashMap::new();
    let mut incoming_messages_by_year: HashMap<i32, usize> = HashMap::new();
    let mut outgoing_messages_by_year: HashMap<i32, usize> = HashMap::new();

    for (datetime, message_type) in &messages {
        let year = datetime.year();
        *messages_by_year.entry(year).or_insert(0) += 1;
        if message_type == "incoming" {
            *incoming_messages_by_year.entry(year).or_insert(0) += 1;
        } else if message_type == "outgoing" {
            *outgoing_messages_by_year.entry(year).or_insert(0) += 1;
        }
    }

    for (year, count) in &messages_by_year {
        println!("{}: {} messages ({} incoming, {} outgoing)", 
            year, 
            count, 
            incoming_messages_by_year.get(year).unwrap_or(&0), 
            outgoing_messages_by_year.get(year).unwrap_or(&0)
        );
    }

    let total_messages = messages.len();
    println!("Total number of messages: {}", total_messages);

    let total_incoming_messages = messages.iter().filter(|(_, t)| t == "incoming").count();
    let total_outgoing_messages = messages.iter().filter(|(_, t)| t == "outgoing").count();

    println!("Total number of incoming messages: {}", total_incoming_messages);
    println!("Total number of outgoing messages: {}", total_outgoing_messages);

    if let Some(earliest_message) = messages.iter().min_by_key(|m| m.0) {
        println!("Earliest date: {}", earliest_message.0.format("%B %e, %Y"));
    }

    Ok(())
}
