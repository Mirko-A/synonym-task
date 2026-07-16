use crate::cli::Cli;
mod cli;

#[tokio::main]
async fn main() {
    let _cli = Cli::parse_args();
}
