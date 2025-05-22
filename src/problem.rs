use anyhow::{Context, Ok};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{path::Path};
use tokio::{self, fs};

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

struct Remote<'a> {
    url: &'a str,
    access_token: &'a str,
}
pub struct ProblemBuilder<'a> {
    local_store: Option<&'a Path>,
    remote: Option<Remote<'a>>,
}

impl<'a> ProblemBuilder<'a> {
    pub fn new() -> Self {
        Self {
            local_store: None,
            remote: None,
        }
    }
    pub fn set_local_store(&mut self, path: &'a Path) {
        self.local_store = Some(path);
    }

    pub fn set_remote(&mut self, url: &'a str, access_token: &'a str) {
        let remote = Remote { url, access_token };
        self.remote = Some(remote);
    }

    pub async fn build(&self) -> anyhow::Result<Problem> {

        const DEFAUT_FILE_PATH:&str = "problem.json";

        async fn fetch_and_store(
            remote: &Remote<'_>,
            local_path: Option<&Path>,
        ) -> anyhow::Result<Problem> {

            let url = format!("{}?access_token={}", remote.url, remote.access_token);

            let client = Client::new();

            let response = client
                .get(&url)
                .send()
                .await
                .with_context(|| format!("sending request to {}", url))?;
            
            let problem = response
                .json::<Problem>()
                .await
                .context("converting response body to json")?;

            if let Some(path) = local_path {

                let problem_json = serde_json::to_string_pretty(&problem)
                    .context("serializing problem as pretty-printed JSON")?;
                fs::write(path, problem_json)
                    .await
                    .context(format!("writing problem to {:?}", path))?;
    
            }

            Ok(problem)

        }

        match (self.local_store, &self.remote) {

            (Some(local_path), remote_opt) => {

                if local_path.try_exists().context("problem.json not found")? {

                    let file_content = fs::read_to_string(local_path)
                        .await
                        .context("reading from problem.json")?;
                    
                    let problem = serde_json::from_str::<Problem>(&file_content)
                        .context("converting file content string into json")?;

                    Ok(problem)
                    
                } else if let Some(remote) = remote_opt {
                    fetch_and_store(remote, Some(local_path)).await
                } else {
                    anyhow::bail!("you have not set local_store or remote");
                }
            }
            (None, Some(remote)) => fetch_and_store(remote, Some(Path::new(DEFAUT_FILE_PATH))).await,
            (None, None) => anyhow::bail!("you have not set local_store or remote"),
        }
    }
}
