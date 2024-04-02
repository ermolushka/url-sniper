use reqwest::{Error, StatusCode};

#[derive(Debug)]
pub struct ResponseData {
    pub code: StatusCode,
    pub length: u64
}

// Asynchronously fetch data from a URL and return the text
pub async fn fetch_data(url: String) -> Result<ResponseData, Error> {
    println!("{}", url);
    let res = reqwest::get(url).await?;
    let res_length: u64 = match res.content_length() {
        Some(v) =>  v,
        None => 0
    };
    let data: ResponseData = ResponseData {code: res.status(), length: res_length};
    Ok(data)
}