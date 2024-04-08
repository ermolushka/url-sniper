use reqwest::{Error, StatusCode};
use std::fmt;

#[derive(Debug)]
pub struct ResponseData {
    pub code: StatusCode,
    pub length: u64,
    pub url: String,
}

impl fmt::Display for ResponseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(code: {}, response length: {}, path: {})",
            self.code,
            self.length,
            self.url
        )
    }
}

// Asynchronously fetch data from a URL and return the text
pub async fn fetch_data(url: String) -> Result<ResponseData, Error> {
    let res = reqwest::get(format!("https://{}", url)).await?;

    let res_length: u64 = match res.content_length() {
        Some(v) => v,
        None => 0,
    };
    let data: ResponseData = ResponseData {
        code: res.status(),
        length: res_length,
        url: url.clone(),
    };
    Ok(data)
}
