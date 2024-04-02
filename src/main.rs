use tokio;
pub mod request;
use crate::request::external_request::fetch_data;
use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::sync::Semaphore;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;
use std::sync::Arc;

#[derive(Parser)]
struct Args {
    file: String,
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let file_path = args.file;
    let base_url = args.url;
    let file = File::open(file_path).await.expect("Failed to open file");
    let reader = BufReader::new(file);

    // Concurrency limit
    let semaphore = Arc::new(Semaphore::new(50)); // Adjust the concurrency limit as needed

    let mut futures = FuturesUnordered::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.expect("Failed to read line") {
        let url = format!("https://{}/{}", base_url, line);
        let permit = semaphore.clone().acquire_owned().await.expect("Failed to acquire semaphore permit");

        futures.push(tokio::spawn(async move {
            let result = fetch_data(url).await;
            drop(permit); // Release the permit as soon as the request is done
            result
        }));

        if futures.len() >= 50 {
            if let Some(result) = futures.next().await {
                // Process result
                match result {
                    Ok(Ok(data)) => println!("Received data: {:?}, url: {}", data, format!("https://{}/{}", base_url, line)),
                    Ok(Err(e)) => println!("Error fetching data: {}", e),
                    Err(e) => println!("Task failed: {:?}", e), // Handling join handle errors
                }
            }
        }
    }

    // Consume any remaining futures
    while let Some(result) = futures.next().await {
        match result {
            Ok(Ok(data)) => println!("Received data: {:?}", data),
            Ok(Err(e)) => println!("Error fetching data: {}", e),
            Err(e) => println!("Task failed: {:?}", e),
        }
    }
}
