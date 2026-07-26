use std::process::Command;
use colored::*;

pub async fn run() -> anyhow::Result<()> {
    println!("{}", "Running PlazaVM Workspace Validation...".bold().blue());

    let steps = vec![
        ("Format Check", "cargo", vec!["fmt", "--all", "--", "--check"]),
        ("Clippy (Lints)", "cargo", vec!["clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"]),
        ("Tests", "cargo", vec!["test", "--workspace", "--all-features"]),
    ];

    let mut all_passed = true;

    for (desc, cmd, args) in steps {
        println!("\n{} Running {}...", "→".cyan(), desc);
        let status = Command::new(cmd).args(&args).status()?;
        
        if status.success() {
            println!("  {}", "✓ Passed".green());
        } else {
            println!("  {}", "✗ Failed".red());
            all_passed = false;
            break; // Stop on first failure
        }
    }

    println!();
    if all_passed {
        println!("{}", "All validations passed successfully!".bold().green());
    } else {
        println!("{}", "Validation failed. Please fix the errors above.".bold().red());
        anyhow::bail!("Validation failed.");
    }

    Ok(())
}
