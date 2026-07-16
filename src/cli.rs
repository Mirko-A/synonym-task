use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long, value_parser = parse_jobs)]
    pub jobs: std::num::NonZeroUsize,
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
