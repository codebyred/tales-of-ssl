use anyhow::{Context, Ok};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequiredData {
    pub domain: String,
    pub serial_number: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Problem {
    pub private_key: String,
    pub required_data: RequiredData,
}

pub async fn get_from(
    client: &Client,
    url: &str
) -> anyhow::Result<Problem> {

    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("sending request to {}", url))?;

    let problem = response
        .json::<Problem>()
        .await
        .context("converting response body to json")?;

    Ok(problem)
}
