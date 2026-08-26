use std::env;
use std::ffi::OsString;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: plaza-guest <command> [args...]");
        return ExitCode::FAILURE;
    }

    let cmd = &args[1];

    if cmd == "agent" {
        println!("SUCCESS_PLAZA_GUEST_READY");
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        use std::io::BufRead;
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(cmd_line) = line {
                let cmd_line = cmd_line.trim();
                if cmd_line.is_empty() {
                    continue;
                }

                // Extremely simple parser for demonstration
                let parts: Vec<&str> = cmd_line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                let program = parts[0];
                let prog_args = &parts[1..];

                match std::process::Command::new(program).args(prog_args).output() {
                    Ok(output) => {
                        std::io::stdout().write_all(&output.stdout).unwrap();
                        std::io::stderr().write_all(&output.stderr).unwrap();
                        println!("EXIT_CODE: {}", output.status.code().unwrap_or(1));
                        std::io::stdout().flush().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Failed to execute command '{}': {}", cmd_line, e);
                        println!("EXIT_CODE: 127");
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
        }
        return ExitCode::SUCCESS;
    } else if cmd == "ready" {
        println!("SUCCESS_PLAZA_GUEST_READY");
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        return ExitCode::SUCCESS;
    } else if cmd == "shutdown" {
        println!("Shutting down guest...");
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        // In a real system, this would make a syscall to shutdown the machine
        // e.g. reboot(LINUX_REBOOT_CMD_POWER_OFF)
        return ExitCode::SUCCESS;
    }

    let app_args: Vec<OsString> = args[1..].iter().map(|s| OsString::from(s)).collect();

    let code = match cmd.as_str() {
        "ls" => uu_ls::uumain(app_args.into_iter()),
        "cat" => uu_cat::uumain(app_args.into_iter()),
        "mkdir" => uu_mkdir::uumain(app_args.into_iter()),
        "rm" => uu_rm::uumain(app_args.into_iter()),
        "cp" => uu_cp::uumain(app_args.into_iter()),
        _ => {
            eprintln!("plaza-guest: command not found: {}", cmd);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::from(code as u8)
}
