use tokio;
pub mod request;
use crate::request::external_request::fetch_data;
use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Semaphore;

#[derive(Parser)]
struct Args {
    file: String,
    url: String,
    #[clap(long = "max-concurrent", short = 'm')]
    max_concurrent: usize,
    #[clap(long = "response-code", short = 'r')]
    response_code_filter: Option<String>,
    #[clap(long = "content-length-filter", short = 'c')]
    content_length_filter: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let file_path = args.file;
    let base_url = args.url;
    let max_concurrent = args.max_concurrent;
    let response_code_filter: Vec<u16> = args
        .response_code_filter
        .map(|s| s.split(",").filter_map(|part| part.parse().ok()).collect())
        .unwrap_or_else(Vec::new);
    let content_length_filter: Vec<u64> = args
        .content_length_filter
        .map(|s| s.split(",").filter_map(|part| part.parse().ok()).collect())
        .unwrap_or_else(Vec::new);
    let file = File::open(file_path).await.expect("Failed to open file");
    let reader = BufReader::new(file);

    // Concurrency limit
    let semaphore = Arc::new(Semaphore::new(max_concurrent)); // Adjust the concurrency limit as needed

    let mut futures = FuturesUnordered::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.expect("Failed to read line") {
        let url = format!("{}/{}", base_url, line);
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("Failed to acquire semaphore permit");

        futures.push(tokio::spawn(async move {
            let result = fetch_data(url).await;
            drop(permit); // Release the permit as soon as the request is done
            result
        }));

        if futures.len() >= max_concurrent {
            if let Some(result) = futures.next().await {
                // Process result
                match result {
                    Ok(Ok(data)) => {
                        if check_filters(
                            &response_code_filter,
                            &content_length_filter,
                            &data.code.as_u16(),
                            &data.length,
                        ) {
                            let res = format!("Received data: {data}");
                            println!("{}", res);
                        }
                    }
                    Ok(Err(e)) => println!("Error fetching data: {}", e),
                    Err(e) => println!("Task failed: {:?}", e), // Handling join handle errors
                }
            }
        }
    }

    // Consume any remaining futures
    while let Some(result) = futures.next().await {
        match result {
            Ok(Ok(data)) => {
                if check_filters(
                    &response_code_filter,
                    &content_length_filter,
                    &data.code.as_u16(),
                    &data.length,
                ) {
                    let res = format!("Received data: {data}");
                    println!("{}", res);
                }
            }
            Ok(Err(e)) => println!("Error fetching data: {}", e),
            Err(e) => println!("Task failed: {:?}", e),
        }
    }
}

fn check_filters(
    response_code_filter: &Vec<u16>,
    content_length_filter: &Vec<u64>,
    code: &u16,
    length: &u64,
) -> bool {
    if (response_code_filter.len() > 0 && response_code_filter.contains(code))
        || (content_length_filter.len() > 0 && content_length_filter.contains(length))
    {
        return false;
    }
    return true;
}
