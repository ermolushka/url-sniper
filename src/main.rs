use tokio;
pub mod request;
pub mod wordlists;
use crate::request::external_request::fetch_data;
use futures::stream::{FuturesUnordered, StreamExt};
use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,

    /// Number of times to greet
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();

    let path = args.file;
    let url = args.url;
    let file = File::open(path).expect("Failed to open file");

    // non-blocking concurrent execution
    let mut futures = FuturesUnordered::new();

    // Process each line from the file
    io::BufReader::new(file).lines().filter_map(Result::ok).for_each(|word| {
        let url = format!("https://{}.{}", word, url);
        futures.push(fetch_data(url));
    });

    // Consume the futures as they are completed
    while let Some(result) = futures.next().await {
        match result {
            Ok(data) => println!("Received data: {:?}", data),
            Err(e) => println!("Error fetching data: {}", e),
        }
    }
}
