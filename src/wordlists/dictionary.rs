use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn process_dict(path: &str) -> io::Result<Vec<String>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Create a buffered reader for efficient reading.
    let reader = BufReader::new(file);
    let mut lines = vec![];

    // Iterate over each line in the file.
    for line_result in reader.lines() {
        // `line_result` is a Result<String, io::Error>, so we handle any potential error using `?`.
        let line = line_result?;

        // Process the line here (for now, we'll just print it).
        lines.push(line)
    }
    print!("{:?}", lines);
    Ok(lines)
}
