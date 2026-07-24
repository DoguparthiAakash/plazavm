//! Diagnostic bundle generation module.

use crate::bootstrap::Container;
use plaza_core::logging::Logger;
use plaza_core::panic_handler::CrashHandler;
use plaza_core::paths;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use zip::write::FileOptions;
use zip::ZipWriter;

pub struct DiagnosticsBundle;

impl DiagnosticsBundle {
    pub async fn generate(container: &Container) -> anyhow::Result<PathBuf> {
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let diag_dir = paths::data_dir().join("diagnostics");
        fs::create_dir_all(&diag_dir)?;

        let zip_path = diag_dir.join(format!("diagnostics_{}.zip", timestamp));
        let file = fs::File::create(&zip_path)?;
        let mut zip = ZipWriter::new(file);

        let options: FileOptions<'_, ()> =
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // 1. Version Info
        let version_info = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "target": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "timestamp": chrono::Local::now().to_rfc3339()
        });
        zip.start_file("version_info.json", options)?;
        zip.write_all(serde_json::to_string_pretty(&version_info)?.as_bytes())?;

        // 2. Platform Profile
        let caps = container.platform.capabilities().await.ok();
        zip.start_file("platform_profile.json", options)?;
        zip.write_all(serde_json::to_string_pretty(&caps)?.as_bytes())?;

        // 3. Plugins Matrix
        let plugins = container.plugin_host.available_runtime_plugins().await;
        let plugin_list: Vec<_> = plugins
            .iter()
            .map(|p| {
                serde_json::json!({
                    "id": p.id(),
                    "name": p.display_name(),
                    "manifest": p.manifest()
                })
            })
            .collect();
        zip.start_file("plugin_matrix.json", options)?;
        zip.write_all(serde_json::to_string_pretty(&plugin_list)?.as_bytes())?;

        // 4. Main Log File
        let log_file = Logger::main_log_file();
        if let Ok(log_content) = fs::read_to_string(log_file) {
            zip.start_file("plazavm.log", options)?;
            zip.write_all(log_content.as_bytes())?;
        }

        // 5. Crash Reports
        let crashes = CrashHandler::list_crash_reports();
        zip.start_file("crash_reports.json", options)?;
        zip.write_all(serde_json::to_string_pretty(&crashes)?.as_bytes())?;

        zip.finish()?;

        Logger::info(&format!(
            "Generated diagnostic bundle at {}",
            zip_path.display()
        ));

        Ok(zip_path)
    }
}
