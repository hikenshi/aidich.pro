use std::fs;
use std::io::prelude::*;
use std::path::Path;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

#[tokio::main]
async fn main() {
    // Read the username and password from the config.cfg file
    let config_path = Path::new("config.cfg");
    let config_contents = fs::read_to_string(config_path).unwrap_or_else(|_| panic!("Failed to read config file: {:?}", config_path));
    let mut config_parts = config_contents.trim().split(',');
    let username = config_parts.next().unwrap_or_default();
    let password = config_parts.next().unwrap_or_default();

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

        // Create a client and set the authentication header
        let client = Client::new();
        let auth_header = format!("Basic {}", base64::encode(&format!("{}:{}", username, password)));

        // Send the content to the FastAPI endpoint
        let response = client
            .get(base_url)
            .query(&[("message", content)])
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

            // Write the result to a file in the "output" folder
            let output_file_path = output_folder.join(file_path.file_name().unwrap());
            let output_file_path_clone = output_file_path.clone();
            let mut output_file = fs::File::create(output_file_path).unwrap();
            output_file.write_all(result.as_bytes()).unwrap();
            println!(
                "Processed {:?} and saved the result to {:?}",
                file_path, output_file_path_clone
            );
        } else {
            println!(
                "Error processing {:?}. Status code: {}",
                file_path,
                response.status()
            );
        }
    }
}