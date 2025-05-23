pub mod certificate;
pub mod models;
pub mod utility;
pub mod request_handler;
use anyhow::Context;
use certificate::CertificateBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use models::{Feedback, Solution};
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

    let handler = request_handler::Handler {
        client
    };

    let problem = handler.get_from(
        &problem_url,
    )
    .await
    .context("fetching problem")?;

    progress_bar.set_message("Problem received");

    let certificate = CertificateBuilder::build(problem).context("creating certificate")?;

    let solution = Solution {
        certificate: certificate.get_value()
    };

    println!("Certificate: {}", certificate.get_value());

    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_style(
    ProgressStyle::with_template("{spinner:.green} {msg}")
        .unwrap()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    progress_bar.set_message("Submitting certificate");
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let feedback: Feedback = handler.post_to(&solution_url, solution).await.context("submitting solution")?;

    progress_bar.finish_with_message("Feedback Received");

    match feedback {
        Feedback::Success { success:_, message } => {
            print!("succes ");
            if let Some(message) = message {
                println!("{}", message)
            }
        }

        Feedback::Rejected { rejected } => {
            println!("{}", rejected);
        }
    }

    Ok(())

}
