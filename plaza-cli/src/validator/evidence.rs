//! Evidence collection harness capturing command executions, logs, telemetry metrics, and raw artifacts.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command: String,
    pub exit_code: i32,
    pub duration_ms: u128,
    pub stdout_path: String,
    pub stderr_path: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageEvidence {
    pub stage_number: u32,
    pub stage_name: String,
    pub log_file: String,
    pub metrics_file: String,
    pub commands: Vec<CommandResult>,
    pub artifacts: Vec<String>,
}

pub struct EvidenceCollector {
    pub base_dir: PathBuf,
}

impl EvidenceCollector {
    pub fn new(base_dir: PathBuf) -> anyhow::Result<Self> {
        let dirs = [
            "build",
            "tests",
            "integration",
            "stress",
            "failure",
            "benchmarks",
            "coverage",
            "security",
            "platform",
            "plugins",
            "screenshots",
            "cli",
            "logs",
            "metrics",
            "json",
            "graphs",
            "reports",
            "dashboard",
        ];

        for d in dirs {
            fs::create_dir_all(base_dir.join(d))?;
        }

        Ok(Self { base_dir })
    }

    /// Execute a command and capture command.txt, stdout.log, stderr.log, exit_code.json, duration.json.
    pub fn execute_and_capture(
        &self,
        sub_folder: &str,
        command_str: &str,
        program: &str,
        args: &[&str],
        cwd: Option<&Path>,
    ) -> CommandResult {
        let folder = self.base_dir.join(sub_folder);
        fs::create_dir_all(&folder).ok();

        let start_instant = Instant::now();

        let mut cmd = Command::new(program);
        cmd.args(args);
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let output_res = cmd.output();
        let duration_ms = start_instant.elapsed().as_millis();

        let (exit_code, stdout_bytes, stderr_bytes) = match output_res {
            Ok(output) => (
                output.status.code().unwrap_or(-1),
                output.stdout,
                output.stderr,
            ),
            Err(e) => (-1, vec![], e.to_string().into_bytes()),
        };

        let slug = command_str
            .replace([' ', '-', '/', '\\', ':', '.'], "_")
            .to_lowercase();

        let stdout_file = folder.join(format!("{slug}_stdout.log"));
        let stderr_file = folder.join(format!("{slug}_stderr.log"));
        let cmd_file = folder.join(format!("{slug}_command.txt"));
        let meta_file = folder.join(format!("{slug}_meta.json"));

        fs::write(&cmd_file, command_str).ok();
        fs::write(&stdout_file, &stdout_bytes).ok();
        fs::write(&stderr_file, &stderr_bytes).ok();

        let meta = serde_json::json!({
            "command": command_str,
            "program": program,
            "args": args,
            "exit_code": exit_code,
            "duration_ms": duration_ms,
            "timestamp": chrono::Local::now().to_rfc3339()
        });
        fs::write(&meta_file, serde_json::to_string_pretty(&meta).unwrap()).ok();

        CommandResult {
            command: command_str.to_string(),
            exit_code,
            duration_ms,
            stdout_path: stdout_file.to_string_lossy().to_string(),
            stderr_path: stderr_file.to_string_lossy().to_string(),
            success: exit_code == 0,
        }
    }

    /// Append a log line to stage log file `logs/stageXX.log`.
    pub fn log_stage_event(&self, stage_number: u32, message: &str) {
        let log_file = self
            .base_dir
            .join("logs")
            .join(format!("stage{:02}.log", stage_number));
        let ts = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
        let line = format!("[{ts}] {message}\n");
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .ok();
        if let Some(ref mut f) = file {
            use std::io::Write;
            let _ = f.write_all(line.as_bytes());
        }
    }

    /// Write telemetry JSON to `metrics/stageXX_metrics.json`.
    pub fn save_stage_metrics(
        &self,
        stage_number: u32,
        metrics: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let metrics_file = self
            .base_dir
            .join("metrics")
            .join(format!("stage{:02}_metrics.json", stage_number));
        fs::write(&metrics_file, serde_json::to_string_pretty(metrics)?)?;
        Ok(metrics_file.to_string_lossy().to_string())
    }

    /// Write raw JSON report to `reports/stageXX_report.json`.
    pub fn save_stage_report(
        &self,
        report_name: &str,
        report: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let path = self
            .base_dir
            .join("reports")
            .join(format!("{report_name}.json"));
        fs::write(&path, serde_json::to_string_pretty(report)?)?;
        Ok(path.to_string_lossy().to_string())
    }
}
