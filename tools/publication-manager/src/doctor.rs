use std::process::Command;
use colored::*;

pub async fn run(verbose: bool) -> anyhow::Result<()> {
    println!("{}", "Running PlazaVM Health Diagnostics...".bold().blue());

    let tools = vec![
        ("rustc", "--version"),
        ("cargo", "--version"),
        ("npm", "--version"),
        ("gh", "--version"),
        ("git", "--version"),
    ];

    let mut all_passed = true;

    for (tool, arg) in tools {
        if verbose {
            println!("Checking {}...", tool);
        }
        
        match Command::new(tool).arg(arg).output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout).lines().next().unwrap_or("").to_string();
                println!("  {} {} ({})", "✓".green(), tool.bold(), version.dimmed());
            }
            _ => {
                println!("  {} {}", "✗".red(), tool.bold().red());
                if verbose {
                    println!("    {} is not installed or not in PATH.", tool);
                }
                all_passed = false;
            }
        }
    }

    println!();
    if all_passed {
        println!("{}", "All required tools are installed!".bold().green());
    } else {
        println!("{}", "Some required tools are missing. Please install them to continue.".bold().red());
        anyhow::bail!("Diagnostics failed.");
    }

    Ok(())
}
