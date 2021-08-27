use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

pub async fn post(
    url: &str,
    obj: impl Serialize,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let result = client
        .post(url)
        .headers(headers)
        .json(&obj)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;

    Ok(result)
}
