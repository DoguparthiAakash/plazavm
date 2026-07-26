use std::process::Command;
use colored::*;

pub async fn bootstrap() -> anyhow::Result<()> {
    println!("{}", "Bootstrapping PlazaVM developer environment...".bold().blue());

    let steps = vec![
        ("Updating Rust", "rustup", vec!["update"]),
        ("Installing cargo-binstall", "cargo", vec!["install", "cargo-binstall", "--locked"]),
        ("Installing tauri-cli", "cargo", vec!["binstall", "tauri-cli", "-y"]),
        ("Installing npm dependencies", "npm", vec!["install"]),
    ];

    for (desc, cmd, args) in steps {
        println!("{} {}", "→".cyan(), desc);
        let status = Command::new(cmd).args(&args).status();
        match status {
            Ok(s) if s.success() => {
                println!("  {}", "✓ Done".green());
            }
            _ => {
                println!("  {}", "✗ Failed".red());
                println!("  Command: {} {:?}", cmd, args);
            }
        }
    }

    println!("\n{}", "Bootstrap complete!".bold().green());
    Ok(())
}

pub async fn update() -> anyhow::Result<()> {
    println!("{}", "Updating PlazaVM dependencies...".bold().blue());
    let status = Command::new("cargo").args(["update"]).status()?;
    if status.success() {
        println!("{}", "✓ Cargo dependencies updated".green());
    } else {
        println!("{}", "✗ Failed to update dependencies".red());
    }
    Ok(())
}
