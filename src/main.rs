pub mod certificate;
pub mod problem;
pub mod utility;
use anyhow::Context;
use certificate::CertificateBuilder;
use problem::{Problem, ProblemBuilder};
use std::{env, path::Path};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenvy::dotenv().context("reading .env file")?;

    let access_token = env::var("ACCESS_TOKEN").context("reading ACCESS_TOKEN from .env")?;

    let mut problem_builder = ProblemBuilder::new();
    
    problem_builder.set_local_store(Path::new("problem.json"));
    problem_builder.set_remote(
        "https://hackattic.com/challenges/tales_of_ssl/problem",
        &access_token,
    );

    let problem = problem_builder.build().await.context("building problem")?;

    CertificateBuilder::build(problem).context("creating certificate")?;

    Ok(())
}
