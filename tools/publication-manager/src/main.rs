pub mod analyzer;
pub mod cli;
pub mod doctor;
pub mod extractor;
pub mod github;
pub mod history;
pub mod installer;
pub mod metadata;
pub mod release;
pub mod repair;
pub mod reporting;
pub mod synchronization;
pub mod templates;
pub mod utilities;
pub mod validator;
pub mod workspace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse CLI arguments
    let args = cli::parse_args();

    // Execute corresponding handler
    cli::execute(args).await
}
