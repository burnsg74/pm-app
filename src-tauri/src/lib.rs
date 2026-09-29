// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;

use std::sync::Mutex;
use sysinfo::System;
use tauri::{Manager, PhysicalPosition, PhysicalSize, Position, Size};

struct SysInfoState(Mutex<System>);

#[derive(Clone, serde::Serialize)]
struct SystemStats {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    memory_usage: f32,
}

#[tauri::command]
fn get_system_stats(state: tauri::State<SysInfoState>) -> SystemStats {
    let mut sys = state.0.lock().unwrap();
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    let memory_usage = if memory_total > 0 {
        (memory_used as f32 / memory_total as f32) * 100.0
    } else {
        0.0
    };

    SystemStats {
        cpu_usage,
        memory_used,
        memory_total,
        memory_usage,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SysInfoState(Mutex::new(System::new_all())))
        .invoke_handler(tauri::generate_handler![
            get_system_stats,
            db::list_tasks,
            db::create_task,
            db::update_task_title,
            db::set_task_status,
            db::delete_task
        ])
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let conn = db::open_file(&dir.join("pm.db"))?;
            app.manage(db::DbState(Mutex::new(conn)));
            if let Some(window) = app.get_webview_window("main") {
                // Get the current monitor the app is initializing on
                if let Ok(Some(monitor)) = window.current_monitor() {
                    // work_area provides dimensions excluding the macOS dock and menu bar.
                    // If you want to cover the dock/menu bar as well, use monitor.size() instead.
                    let area = monitor.work_area();

                    // 1. Instantly position the window at the top-left of the work area
                    let _ = window.set_position(Position::Physical(PhysicalPosition {
                        x: area.position.x,
                        y: area.position.y,
                    }));

                    // 2. Instantly change the physical window size to match the space
                    let _ = window.set_size(Size::Physical(PhysicalSize {
                        width: area.size.width,
                        height: area.size.height,
                    }));
                }

                // 3. Reveal the window once layout calculations are finished
                let _ = window.show();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
