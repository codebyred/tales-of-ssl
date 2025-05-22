use crate::certificate::Certificate;
use anyhow::Context;
use reqwest::Client;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Solution {
    certificate: String,
}

pub async fn submit_to(client: &Client, certificate: Certificate, url: &str) -> anyhow::Result<()> {
    let solution = Solution {
        certificate: certificate.get_value(),
    };

    let response = client
        .post(url)
        .json(&solution)
        .send()
        .await
        .with_context(|| String::from("Failed to send POST request with certificate"))?;


    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Submission failed with status {}: {}", status, text);
    }

    println!("✅ Certificate submitted successfully!");
    Ok(())
}
