use std::time::Duration;

use crate::cli::Cli;
use crate::job::Job;
use crate::runtime::Runtime;

mod cli;
mod job;
mod runtime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    let total_jobs = cli.jobs.get();
    let fail_count = fail_count(total_jobs, cli.fail_rate);
    let inputs = (1..=total_jobs)
        .map(|id| CliJobInput {
            id,
            should_fail: id <= fail_count,
        })
        .collect();

    match Runtime::start(inputs, || CliJob).await {
        Ok(completed) => {
            tracing::info!("completed {} jobs", completed.len());
        }
        Err(error) => {
            tracing::error!(%error, "runtime error");
            return Err(anyhow::anyhow!("runtime error: {error}"));
        }
    }

    Ok(())
}

struct CliJob;

struct CliJobInput {
    id: usize,
    should_fail: bool,
}

impl Job for CliJob {
    type Input = CliJobInput;
    type Output = usize;
    type Error = String;

    async fn run(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.should_fail {
            return Err(format!("job {} failed", input.id));
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
        tracing::info!("job {} completed", input.id);
        Ok(input.id)
    }
}

fn fail_count(total_jobs: usize, fail_rate: f64) -> usize {
    ((total_jobs as f64) * fail_rate).round() as usize
}
