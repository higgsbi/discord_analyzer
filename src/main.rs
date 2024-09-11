use std::{fmt::format, fs};
use clap::Parser;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    channels: bool,
}

fn main() {
    let args = Args::parse();
    
    let root_path = args.path.unwrap_or("package/".to_owned());
    let channel_path = root_path.clone() + "/messages/index.json";
    let output_path = args.output.unwrap_or("message_history.md".to_owned());

    let channel_string: String = fs::read_to_string(channel_path)
        .expect("Could not open channel file. Ensure \"package\" directory is present");
    let channel_json: Value = serde_json::from_str(&channel_string)
        .expect("Could not parse channel json");

    let mut channels: Vec<(String, String)> = channel_json.as_object()
        .expect("Messages index data could not be read")
        .iter()
        .map(|(id, channel)| -> (String, String) {
            let channel_name: String = if channel.to_string().contains("Unknown") {
                serde_json::to_string_pretty(&channel)
                    .expect("Could not format message string")
                    .replace("Unknown", "Inaccessible")
            } else {
                serde_json::to_string_pretty(&channel)
                    .expect("Could not format message string")
            };
            (id.to_owned(), channel_name)
        }).collect();

    let message_history: String = channels.iter().map(|(channel_id, channel_name)| {
        get_message_history(&root_path, channel_id, channel_name) 
    }).collect();
 
    if args.channels {
        channels.sort_by(|a, b| a.1.cmp(&b.1));
        channels.iter().for_each(|(id, name)| {
            if name.contains("Direct") {
                let url = format!("https://discord.com/channels/@me/{}", id.replace("\"", ""));
                println!("{} ({})", name.replace("\"", ""), url);
            } else {
        
                println!("{} ({})", name.replace("\"", ""), id.replace("\"", ""));
            }
            
        });
    }

    fs::write(output_path, message_history).expect("Unable to write message_history");
    println!("\nFinished discord message analysis");
    
}

// creates a formatted string representation of the user's entire message history
fn get_message_history(root_path: &str, channel_id: &str, channel_name: &str) -> String {
    let prefix = format!("-----------{} ({})-----------\n\n", 
        &channel_name.replace("\"", ""), 
        &channel_id.replace("\"", ""));
    
    let messages_path = format!("{}/messages/c{}/messages.json", root_path, channel_id);
    let messages_string = fs::read_to_string(&messages_path)
        .expect(&format!("Could not read messages for folder {}", &messages_path)); 
    let messages_json: Value = serde_json::from_str(&messages_string)
        .expect(&format!("Could not read json for channel c{}", &channel_id));

    prefix + &messages_json.as_array().iter().map(|chain| -> String {
        chain.iter()
            .filter_map(|object| object.as_object())
            .filter_map(|map| map.get("Contents"))
            .map(|contents| contents.to_string())
            .filter_map(|contents_string| {
                if contents_string.is_empty() || contents_string.eq("\"\"") {
                    None
                } else {
                    Some(contents_string.replace("\\n", "\n") + "\n")
                }
            }).collect()
    }).collect::<String>()
}
