use crate::cli::Cli;
use crate::job::Job;
use crate::runtime::Runtime;

mod cli;
mod job;
mod runtime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    let inputs = (1..=cli.jobs.get()).collect();

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

impl Job for CliJob {
    type Input = usize;
    type Output = ();
    type Error = String;

    async fn run(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        tracing::info!("job {input} completed");
        Ok(())
    }
}
