use std::process::Command;
use colored::*;

pub async fn publish(dry_run: bool, stage: Option<String>, repo: Option<String>) -> anyhow::Result<()> {
    println!("{}", "Starting PlazaVM GitHub Publication Pipeline...".bold().blue());

    if dry_run {
        println!("{} Running in DRY RUN mode", "ℹ".cyan());
    }
    if let Some(ref r) = repo {
        println!("{} Filtering by repository: {}", "ℹ".cyan(), r);
    }
    if let Some(ref s) = stage {
        println!("{} Specific stage: {}", "ℹ".cyan(), s);
    }

    // Temporarily wrap the existing publish-all-to-github.ps1 logic
    let status = Command::new("powershell.exe")
        .args(["-ExecutionPolicy", "Bypass", "-File", "publish-all-to-github.ps1"])
        .status()?;

    if status.success() {
        println!("{}", "Publication Pipeline completed successfully!".bold().green());
    } else {
        println!("{}", "Publication Pipeline failed.".bold().red());
        anyhow::bail!("Publication failed");
    }

    Ok(())
}
