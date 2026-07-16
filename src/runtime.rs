use crate::job::Job;

pub struct Runtime;

impl Runtime {
    pub async fn start<F, J, I, O, E>(inputs: Vec<I>, make_job: F) -> Result<Vec<O>, Error<E>>
    where
        F: Fn() -> J + Send + Sync + 'static,
        J: Job<Input = I, Output = O, Error = E> + Send + Sync + 'static,
        I: Send + Sync + 'static,
        O: Send + Sync + 'static,
        E: Send + Sync + std::fmt::Display + 'static,
    {
        let mut jobs = tokio::task::JoinSet::new();
        let num_jobs = inputs.len();

        tracing::info!(%num_jobs, "starting runtime");

        for input in inputs {
            let job = make_job();
            jobs.spawn(async move { job.run(input).await });
        }

        let mut outputs = Vec::with_capacity(num_jobs);

        // Do not re-run jobs for now, since their error type does not distinguish
        // fatal vs non-fatal errors. Jobs can perform retries internally and return
        // on fatal errors.
        //
        // A better approach would be to have a `job::Error` enum to make that distinction
        // and have the runtime re-run jobs as needed.
        while let Some(outcome) = jobs.join_next().await {
            match outcome {
                Ok(Ok(output)) => outputs.push(output),
                Ok(Err(error)) => {
                    tracing::warn!(%error, "critical job error");
                    abort_jobs(jobs).await.map_err(Error::Runtime)?;
                    return Err(Error::Job(error));
                }
                Err(error) => {
                    let error = anyhow::anyhow!("job task failed to join: {error}");
                    tracing::warn!(%error, "critical job error");
                    abort_jobs(jobs).await.map_err(Error::Runtime)?;
                    return Err(Error::Runtime(error));
                }
            }
        }

        tracing::info!("jobs completed successfully");

        Ok(outputs)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error<E> {
    #[error("job failed: {0}")]
    Job(E),
    #[error("runtime failed: {0}")]
    Runtime(anyhow::Error),
}

async fn abort_jobs<T>(mut jobs: tokio::task::JoinSet<T>) -> anyhow::Result<()>
where
    T: Send + Sync + 'static,
{
    jobs.abort_all();

    while let Some(outcome) = jobs.join_next().await {
        match outcome {
            Err(error) if !error.is_cancelled() => {
                tracing::warn!(%error, "job cleanup failed");
                return Err(anyhow::anyhow!("job cleanup failed: {error}"));
            }
            _ => (),
        }
    }

    Ok(())
}
