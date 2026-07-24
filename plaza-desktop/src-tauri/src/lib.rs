//! Tauri v2 desktop application entry.

pub mod commands;

use plaza_api::AppState;
use tauri::Manager;

pub fn run() {
    tracing_subscriber::fmt::init();

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
