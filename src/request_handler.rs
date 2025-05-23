use anyhow::Context;
use reqwest::Client;

pub struct Handler {
    pub client: Client,
}

impl Handler {
    pub async fn get_from<T>(&self, url: &str) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| format!("sending request to {}", url))?;

        let problem = response
            .json::<T>()
            .await
            .context("converting response body to json")?;

        Ok(problem)
    }

    pub async fn post_to<T,K>(&self, url: &str, body: T) -> anyhow::Result<K>
    where
        T: serde::Serialize,
        K: serde::de::DeserializeOwned
    {

        let response = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .with_context(|| String::from("Failed to send POST request with certificate"))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Submission failed with status {}: {}", status, text);
        }

        let result = response
            .json::<K>()
            .await
            .context("converting response body to json")?;

        Ok(result)
    }
}
