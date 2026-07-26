use std::process::Command;
use colored::*;

pub async fn run() -> anyhow::Result<()> {
    println!("{}", "Starting PlazaVM Repository Extraction Pipeline...".bold().blue());

    // Temporarily wrap the existing publish-all.ps1 logic
    let status = Command::new("powershell.exe")
        .args(["-ExecutionPolicy", "Bypass", "-File", "tools/publication-manager/publish-all.ps1"])
        .status()?;

    if status.success() {
        println!("{}", "Extraction Pipeline completed successfully!".bold().green());
    } else {
        println!("{}", "Extraction Pipeline failed.".bold().red());
        anyhow::bail!("Extraction failed");
    }

    Ok(())
}
