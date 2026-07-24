//! Tauri v2 desktop application entry.

pub mod commands;

use plaza_api::AppState;
use plaza_core::logging::Logger;
use plaza_core::panic_handler::CrashHandler;
use tauri::Manager;

pub fn run() {
    CrashHandler::init();
    tracing_subscriber::fmt::init();
    Logger::info("PlazaVM Desktop Shell started");

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(state) = AppState::initialize().await {
                    app_handle.manage(state);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_workspaces,
            commands::create_workspace,
            commands::start_workspace,
            commands::stop_workspace,
            commands::get_system_metrics,
            commands::get_platform_info,
            commands::list_plugins,
            commands::check_updates,
            commands::generate_diagnostics_bundle,
            commands::open_log_folder,
            commands::get_crash_reports,
            commands::export_config,
            commands::import_config,
            commands::reset_config,
            commands::check_system_readiness,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
