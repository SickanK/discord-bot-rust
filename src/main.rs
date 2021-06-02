use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct HapsyRequest {
    fileid: String,
    creation_date: String,
    modified_date: String,
    chunks: u32,
    size: u32,
    filetype: String,
    filename: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let req_url = "https://cdn.hapsy.net/api/v1/ee9fefea-8cf0-424f-9f7f-cf001e6ce4f1";
    let response = reqwest::get(req_url).await?.text().await?.to_string();
    let json: HapsyRequest = serde_json::from_str(&response).unwrap();

    println!("{:?}", json);

    Ok(())
}
