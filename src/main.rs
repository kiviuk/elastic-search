use serde_json::json;
use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();

    for line in input.lock().lines() {
        let line = line.expect("Failed to read input");

        // Skip if the line doesn't start with [
        if !line.starts_with("[") {
            continue;
        }

        let parts: Vec<&str> = line.split("] ").collect();
        let times: Vec<&str> = parts[0][1..].split("s -> ").collect();
        let text = parts[1];

        let start_time = times[0];
        let end_time = times[1];

        let data = json!({
            "start_time": start_time,
            "end_time": end_time,
            "text": text
        });

        let index_instruction = json!({"index": {"_index": "gsp"}});

        println!("{}", serde_json::to_string(&index_instruction).unwrap());
        println!("{}", serde_json::to_string(&data).unwrap());
    }
}
