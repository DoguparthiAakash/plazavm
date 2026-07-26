use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "plaza-dev", about = "PlazaVM Developer Toolchain", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Bootstrap the developer workspace
    Bootstrap,
    
    /// Run diagnostics on the workspace
    Doctor {
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Validate code quality and tests
    Validate,
    
    /// Extract repositories into staging area
    Extract,
    
    /// Publish staged repositories to GitHub
    Publish {
        #[arg(long)]
        dry_run: bool,
        
        #[arg(long)]
        stage: Option<String>,
        
        #[arg(long)]
        repository: Option<String>,
    },
    
    /// Manage releases and versioning
    Release {
        #[arg(long)]
        alpha: bool,
        
        #[arg(long)]
        beta: bool,
        
        #[arg(long)]
        rc: bool,
    },
    
    /// Update dependencies and upstream
    Update,
    
    /// Attempt safe automatic repairs
    Repair,
    
    /// Clean artifacts and temporary files
    Clean,
    
    /// Analyze workspace and dependencies
    Analyze,
    
    /// Output dependencies map
    Dependencies,
    
    /// Output workspace metadata
    Metadata,
    
    /// Generate comprehensive report
    Report,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

pub async fn execute(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Bootstrap => crate::synchronization::bootstrap().await,
        Commands::Doctor { verbose } => crate::doctor::run(verbose).await,
        Commands::Validate => crate::validator::run().await,
        Commands::Extract => crate::extractor::run().await,
        Commands::Publish { dry_run, stage, repository } => crate::github::publish(dry_run, stage, repository).await,
        Commands::Release { alpha, beta, rc } => crate::release::run(alpha, beta, rc).await,
        Commands::Update => crate::synchronization::update().await,
        Commands::Repair => crate::repair::run().await,
        Commands::Clean => crate::utilities::clean().await,
        Commands::Analyze => crate::analyzer::run().await,
        Commands::Dependencies => crate::analyzer::dependencies().await,
        Commands::Metadata => crate::metadata::run().await,
        Commands::Report => crate::reporting::run().await,
    }
}
