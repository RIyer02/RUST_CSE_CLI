use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct InputData {
    folder_path: Option<String>,
    query: Option<String>,
    url: String,
}

fn read_file(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn send_data_to_url(url: &str, data: &str) -> reqwest::Result<()> {
    let client = Client::new();
    let response = client.post(url).body(data.to_owned()).send()?;
    println!("Status Code: {}", response.status());
    println!("Response Content: {:?}", response.text()?);
    Ok(())
}

fn process_folder(folder_path: &str, url: &str) {
    if let Ok(entries) = std::fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_path) = entry.path().to_str() {
                    match read_file(Path::new(file_path)) {
                        Ok(data) => {
                            if let Err(err) = send_data_to_url(url, &data) {
                                eprintln!("Error sending data for {}: {:?}", file_path, err);
                            }
                        }
                        Err(err) => eprintln!("Error reading {}: {:?}", file_path, err),
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading folder: {:?}", folder_path);
    }
}

fn process_query(query: &str, url: &str) {
    // Implement your logic for handling the query
    // For example, you can send the query directly to the URL
    let response = reqwest::blocking::get(&format!("{}?query={}", url, query));

    match response {
        Ok(res) => {
            println!("Status Code: {}", res.status());
            println!("Response Content: {:?}", res.text());
        }
        Err(err) => eprintln!("Error sending query: {:?}", err),
    }
}

fn main() {
    // Get JSON input from command line
    let args: Vec<String> = std::env::args().collect();
    let json_input = args.get(1).expect("No JSON input provided");

    // Parse JSON input
    let input_data: InputData = serde_json::from_str(&json_input).expect("Failed to parse JSON input");

    // Check if folder_path is present for folder processing
    if let Some(folder_path) = input_data.folder_path {
        process_folder(&folder_path, &input_data.url);
    }

    // Check if query is present for query processing
    if let Some(query) = input_data.query {
        process_query(&query, &input_data.url);
    }
}
