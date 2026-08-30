//! Tauri v2 desktop application entry.

pub mod commands;
pub mod ipc;
pub mod notifications;
pub mod tray;
pub mod window;

use plaza_api::AppState;
use plaza_foundation::core::logging::Logger;
use plaza_foundation::core::panic_handler::CrashHandler;
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
            commands::delete_workspace,
            commands::get_platform_info,
            commands::check_system_readiness,
            commands::open_log_folder,
            commands::get_workspace_config,
            commands::save_workspace_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
