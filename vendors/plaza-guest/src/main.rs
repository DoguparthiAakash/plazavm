use std::env;
use std::process::ExitCode;
use std::ffi::OsString;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: plaza-guest <command> [args...]");
        return ExitCode::FAILURE;
    }

    let cmd = &args[1];
    
    // We must pass the program name as the first argument to the utility
    // So we pass args[1..] mapped to OsString.
    // e.g. plaza-guest ls -la -> ls -la
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
