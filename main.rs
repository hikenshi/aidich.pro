use std::fs;
use std::io::prelude::*;
use std::path::Path;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

fn split_at_positions(string: &str, num_chunks: usize) -> Vec<String> {
    let split_list: Vec<&str> = string.split('\n').collect();
    let mut chunk_size = split_list.len() / num_chunks;
    let mut tach_cau_dau_hieu = '\n';

    if chunk_size == 0 {
        if string.contains('.') {
            let split_list: Vec<&str> = string.split('.').collect();
            chunk_size = split_list.len() / num_chunks;
            tach_cau_dau_hieu = '.';
        } else if string.contains('?') {
            let split_list: Vec<&str> = string.split('?').collect();
            chunk_size = split_list.len() / num_chunks;
            tach_cau_dau_hieu = '?';
        } else {
            return split_list.into_iter().map(|s| s.to_string()).collect();
        }
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    while start < split_list.len() {
        let end = (start + chunk_size).min(split_list.len());
        let chunk = split_list[start..end].join(&tach_cau_dau_hieu.to_string());
        chunks.push(chunk);
        start = end;
    }

    if split_list.len() % num_chunks != 0 {
        let last_chunk = chunks.pop().unwrap();
        let second_last_chunk = chunks.pop().unwrap();
        chunks.push(format!("{}{}{}", second_last_chunk, tach_cau_dau_hieu, last_chunk));
    }

    chunks
}

#[tokio::main]
async fn main() {
    // Read the username, password, and activate_beta from the config.cfg file
    let config_path = Path::new("config.cfg");
    let config_contents = fs::read_to_string(config_path).unwrap_or_else(|_| panic!("Failed to read config file: {:?}", config_path));
    let mut config_parts = config_contents.trim().split(',');
    let username = config_parts.next().unwrap_or_default();
    let password = config_parts.next().unwrap_or_default();
    let activate_beta = config_parts.next().map(|s| s.to_lowercase() == "true").unwrap_or(false);

    // Set the base URL of your FastAPI endpoint
    let base_url = "https://aidich.pro/protected";

    // Create the "output" folder if it doesn't exist
    let output_folder = Path::new("output");
    if !output_folder.exists() {
        std::fs::create_dir(output_folder).unwrap();
    }

    // Get a list of all text files in the current folder
    let text_files: Vec<_> = fs::read_dir(".")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|res| {
            if let Ok(path) = res {
                path.is_file() && path.extension().unwrap_or_default() == "txt"
            } else {
                false
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // Process each text file
    for file_path in text_files {
        // Read the content of the text file
        let content = fs::read_to_string(&file_path).unwrap();
        println!("{}", content);

        // Check if the content needs to be split
        let list_send_to_api = if content.chars().count() > 1980 {
            let chunk_size = std::cmp::max(1, content.chars().count() / 1000);
            split_at_positions(&content, chunk_size)
        } else {
            vec![content]
        };

        // Create the output file for the current text file
        let output_file_path = output_folder.join(file_path.file_name().unwrap());
        let mut output_file = fs::File::create(&output_file_path).unwrap();

        // Process each chunk and append the response to the output file
        for chunk in list_send_to_api {
            // Create a client and set the authentication header
            let client = Client::new();
            let auth_header = format!("Basic {}", base64::encode(&format!("{}:{}", username, password)));

            // Send the chunk to the FastAPI endpoint
            let response = client
                .get(base_url)
                .query(&[("message", chunk), ("activate_beta", activate_beta.to_string())])
                .header(AUTHORIZATION, auth_header)
                .send()
                .await
                .unwrap();

            // Check if the request was successful
            if response.status().is_success() {
                // Extract the response body as a string
                let response_text = response.text().await.unwrap();
                // Parse the JSON response
                let json: Value = serde_json::from_str(&response_text).unwrap();
                // Extract the "message" field from the JSON
                let result = json["message"].as_str().unwrap_or_default();
                // Append the result to the output file
                writeln!(output_file, "{}", result).unwrap();
                println!("Processed chunk and saved the result to {:?}", output_file_path);
            } else {
                println!("Error processing chunk. Status code: {}", response.status());
            }
        }

        println!("Processed {:?} and saved the result to {:?}", file_path, output_file_path);
    }
}