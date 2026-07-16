use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long, value_parser = parse_jobs)]
    pub jobs: std::num::NonZeroUsize,

    #[arg(long, value_parser = parse_fail_rate)]
    pub fail_rate: f64,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

fn parse_jobs(value: &str) -> Result<std::num::NonZeroUsize, String> {
    value
        .parse::<usize>()
        .map_err(|err| format!("invalid job count: {err}"))
        .map(std::num::NonZeroUsize::new)?
        .ok_or_else(|| String::from("job count must not be zero"))
}

fn parse_fail_rate(value: &str) -> Result<f64, String> {
    let fail_rate = value
        .parse::<f64>()
        .map_err(|err| format!("invalid fail rate: {err}"))?;

    match fail_rate {
        0.0..=1.0 => Ok(fail_rate),
        _ => Err(String::from("fail rate must be in the [0.0, 1.0] range")),
    }
}
