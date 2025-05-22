pub mod certificate;
pub mod problem;
pub mod solution;
pub mod utility;
use anyhow::Context;
use certificate::CertificateBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{env, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("reading .env file")?;

    let remote = env::var("REMOTE").context("reading REMOTE from .env")?;

    let access_token = env::var("ACCESS_TOKEN").context("reading ACCESS_TOKEN from .env")?;

    let problem_url = format!("{}/problem?access_token={}", &remote, &access_token);

    let solution_url = format!("{}/solve?access_token={}", &remote, &access_token);

    let progress_bar = ProgressBar::new_spinner();

    progress_bar.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );

    progress_bar.set_message("Fetching problem...");
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let client = Client::new();

    let problem = problem::get_from(
        &client,
        &problem_url,
    )
    .await
    .context("fetching problem")?;

    progress_bar.finish_with_message("Problem received");

    let certificate = CertificateBuilder::build(problem).context("creating certificate")?;

    solution::submit_to(&client, certificate, &solution_url).await.context("submitting solution")?;

    Ok(())
}
