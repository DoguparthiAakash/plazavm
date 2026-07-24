pub mod validator;

use clap::{Parser, Subcommand};
use plaza_api::AppState;
use plaza_core::id::WorkspaceId;
use plaza_workspace::model::WorkspaceSpec;

#[derive(Parser)]
#[command(name = "plaza", author, version, about = "PlazaVM Workspace Platform CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage Workspaces
    Workspace {
        #[command(subcommand)]
        action: WorkspaceAction,
    },
    /// Inspect Host Platform Capabilities
    Platform,
    /// System Information
    System,
    /// Run Full Automated 16-Stage Validation & Snapshot Pipeline
    Validate,
}

#[derive(Subcommand)]
enum WorkspaceAction {
    /// List all workspaces
    List,
    /// Create a new workspace
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        image: Option<String>,
    },
    /// Start a workspace
    Start { id: String },
    /// Stop a workspace
    Stop { id: String },
    /// Delete a workspace
    Delete { id: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let state = AppState::initialize().await?;

    match cli.command {
        Commands::Workspace { action } => match action {
            WorkspaceAction::List => {
                let workspaces = state.workspace_service.list_workspaces().await?;
                println!("Workspaces ({}):", workspaces.len());
                for ws in workspaces {
                    println!(
                        "  - [{}] {} ({:?}, {})",
                        ws.id, ws.name, ws.status.state, ws.status.health
                    );
                }
            }
            WorkspaceAction::Create { name, image } => {
                let mut spec = WorkspaceSpec::default();
                if let Some(img) = image {
                    spec.runtime.image = Some(img);
                }
                let ws = state
                    .workspace_service
                    .create_workspace(&name, spec)
                    .await?;
                println!("Created workspace: {} [{}]", ws.name, ws.id);
            }
            WorkspaceAction::Start { id } => {
                let ws_id = WorkspaceId::parse(&id)?;
                state
                    .workspace_service
                    .set_desired_state(&ws_id, plaza_workspace::model::DesiredState::Running)
                    .await?;
                if let Some(ws) = state.workspace_service.get_workspace(&ws_id).await? {
                    state.controller.reconcile_workspace(&ws).await?;
                }
                println!("Triggered start for workspace {id}");
            }
            WorkspaceAction::Stop { id } => {
                let ws_id = WorkspaceId::parse(&id)?;
                state
                    .workspace_service
                    .set_desired_state(&ws_id, plaza_workspace::model::DesiredState::Stopped)
                    .await?;
                if let Some(ws) = state.workspace_service.get_workspace(&ws_id).await? {
                    state.controller.reconcile_workspace(&ws).await?;
                }
                println!("Triggered stop for workspace {id}");
            }
            WorkspaceAction::Delete { id } => {
                let ws_id = WorkspaceId::parse(&id)?;
                state.workspace_service.delete_workspace(&ws_id).await?;
                println!("Deleted workspace {id}");
            }
        },
        Commands::Platform => {
            let caps = state.platform.capabilities().await?;
            let profile = state.platform.profile().await;
            println!("Host OS: {} {}", caps.os.name, caps.os.version);
            println!(
                "CPU: {} ({} logical cores)",
                caps.cpu.model, caps.cpu.cores_logical
            );
            println!("Memory: {} MB total", caps.memory.total_mb);
            println!("Platform Profile: {profile}");
            println!("Installed Runtimes:");
            for r in caps.installed_runtimes {
                println!("  - {} v{} at {}", r.name, r.version, r.path.display());
            }
        }
        Commands::System => {
            let sample = state.monitor.sample();
            println!("System Metrics:");
            println!("  CPU Usage: {:.1}%", sample.cpu_usage_pct);
            println!(
                "  Memory Usage: {} / {} MB ({:.1}%)",
                sample.memory_used_mb, sample.memory_total_mb, sample.memory_usage_pct
            );
        }
        Commands::Validate => {
            validator::ValidationPipeline::run().await?;
        }
    }

    Ok(())
}
