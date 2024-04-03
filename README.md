# url-sniper

`url-sniper` is a comprehensive tool designed for security researchers, web developers, and IT professionals. It facilitates the exploration and auditing of web domains by checking URL domain paths and subdomains efficiently. Written in Rust, `url-sniper` offers exceptional speed and reliability, ensuring your web investigations are thorough and effective.

## Features

- **Subdomain Discovery**: Quickly identifies and lists subdomains related to your target domain.
- **URL Path Checking**: Scans for valid and accessible paths within a domain, aiding in uncovering hidden directories or files.
- **Multi-threaded Processing**: Utilizes Rust's powerful concurrency features for fast and efficient scans.
- **Custom Status Code Handling**: Allows users to specify which HTTP status codes should be considered during scans, enabling focused and customized investigations.
- **Input from File**: Enables batch processing of URLs and subdomains through input from a text file, streamlining the workflow for large-scale scans.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. If not, visit [Rust's installation page](https://www.rust-lang.org/tools/install) for guidance on setting up the Rust toolchain.

### Installation

Clone the `url-sniper` repository to your local machine:

```bash
git clone https://github.com/yourgithubusername/url-sniper.git
cd url-sniper
```
Build the project using Cargo:
```bash
cargo build
```
This command compiles the project and generates an executable in the target/debug directory.

### Usage
To run url-sniper, use the following syntax:
```bash
cargo run -- [input_file] [your_url] [concurrency_level] [target_status_code]
```
- `input_file`: Path to a text file containing URLs or subdomains to check.
- `your_url`: The base URL or domain you wish to investigate.
- `concurrency_level`: How many threads url-sniper should use concurrently. Higher numbers increase speed but consume more system resources.
- `target_status_code`: The HTTP status code url-sniper will look for. Common codes include 200 for success or 404 for not found.
### Example
```bash
cargo run -- test.txt yourdomain.com 50 404
```
This command checks URLs or subdomains listed in test.txt against yourdomain.com, using 50 threads, and looks for paths that return a 404 status code.

### Contributing
Contributions to url-sniper are welcome! Whether it's feature requests, bug reports, or code contributions, please feel free to make an issue or pull request on GitHub.
### License
`url-sniper` is released under the MIT License.

