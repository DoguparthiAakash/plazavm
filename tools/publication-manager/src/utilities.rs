use std::process::Command;
use colored::*;
use std::fs;
use std::path::Path;

pub async fn clean() -> anyhow::Result<()> {
    println!("{}", "Cleaning PlazaVM developer workspace...".bold().blue());

    // 1. Cargo clean
    println!("{} Cleaning Cargo target...", "→".cyan());
    Command::new("cargo").args(["clean"]).status()?;
    println!("  {}", "✓ Cargo clean complete".green());

    // 2. Remove staging directory
    println!("{} Removing staging directory...", "→".cyan());
    let staging_path = Path::new("staging");
    if staging_path.exists() {
        fs::remove_dir_all(staging_path)?;
        println!("  {}", "✓ Staging directory removed".green());
    } else {
        println!("  {}", "✓ No staging directory found".dimmed());
    }

    println!("\n{}", "Workspace clean complete!".bold().green());
    Ok(())
}
