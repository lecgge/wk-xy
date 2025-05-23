use std::collections::HashMap;
use std::thread::sleep;
use std::time::SystemTime;
use tokio::task;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod can_module;
mod can_matrix;
mod arxml_bean;
mod use_time;

use chrono::{DateTime, Local, TimeZone, Utc};
use tauri::{Manager, WindowEvent};
use crate::use_time::{calculate_usage, load, save};

static mut CAN_MODULE: Option<can_module::CanModule> = None;
pub static mut START_TIME: Option<DateTime<Local>> = None;

#[tauri::command]
async fn start(device: &str) -> Result<(), String> {
    // sleep(std::time::Duration::from_secs(10));
    println!("Hello, {}!", device);
    let mut can_matrix = can_matrix::CanMatrix::new();
    if device == "2712" {
        let _ = can_matrix.load_from_arxml("./resource/output.json");
    } else if device == "ZXD" {
        let _ = can_matrix.load_from_arxml("./resource/output.json");
    } else {
        let _ = can_matrix.load_from_arxml("./resource/output.json");
    }

    task::spawn(async move {
        let module = can_module::start(can_matrix, "vcan0").await.map_err(|e| e.to_string())?;
        unsafe {
            CAN_MODULE = Some(module);
        }
        // 可选：保存到状态中
        // *state.module.lock().unwrap() = Some(module);
        Ok::<(), String>(())
    });
    Ok(())
}

#[tauri::command]
async fn send_message(id: u32, data: Vec<u8>, periodMs: i64,  isFd:bool) -> Result<(), String> {
    let message = can_module::CanMessage {
        id,
        data,
        period_ms:periodMs,
        is_fd:isFd,
        next_send_time: None,
    };
    task::spawn(async move {
        let module = unsafe {
            CAN_MODULE.as_mut().unwrap()
        };
        module.add_periodic_message(message).await;
        Ok::<(), String>(())
    });
    Ok(())
}

#[tauri::command]
async fn send_signal(clusterName: String,id: u32, signalName: String, value: f32) -> Result<(), String> {

    task::spawn(async move {
        let module = unsafe {
            CAN_MODULE.as_mut().unwrap()
        };
        module.send_signal(clusterName, id as i32, &*signalName, value).await;
        Ok::<(), String>(())
    });
    Ok(())
}

#[tauri::command]
async fn send_signals(clusterName: String,id: u32, signals: HashMap<String, f32>) -> Result<(), String> {
    task::spawn(async move {
        let module = unsafe {
            CAN_MODULE.as_mut().unwrap()
        };
        module.send_signals(clusterName, id as i32, signals).await;
        Ok::<(), String>(())
    });
    Ok(())
}

#[tauri::command]
async fn stop_send_message(id: u32) -> Result<(), String> {
    task::spawn(async move {
        let module = unsafe {
            CAN_MODULE.as_mut().unwrap()
        };
        module.stop_send_message(id).await;
        Ok::<(), String>(())
    });
    Ok::<(), String>(())
}
#[tauri::command]
fn stop() {
    unsafe {
        if let Some(mut module) = CAN_MODULE.take() {
            // 正确释放资源
            println!("Module stopped");
            module.drop();
            std::mem::drop(module);
        }
    }
}


#[tauri::command]
fn get_history_time() -> Option<HashMap<String, i64>> {
    Some(calculate_usage(&load().unwrap()))
}
#[tauri::command]
fn get_runtime() -> Result<String, String> {
    unsafe {
        if let Some(start) = START_TIME {
            let now = Local::now();

            Ok(format!("{}", now.format("%Y-%m-%d %H:%M:%S")))
        } else {
            let date_time: DateTime<Local> = Local.with_ymd_and_hms(2025, 01, 01, 00, 00, 00).unwrap();
            let formatted = format!("{}", date_time.format("%d/%m/%Y %H:%M"));
            Ok(formatted)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    unsafe {
        START_TIME = Some(Local::now());
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start,
            stop,
            get_runtime,
            get_history_time,
            send_message,
            send_signal,
            send_signals,
            stop_send_message,
        ])
        .setup(|app| {
            // 监听主窗口关闭事件
            let handler = app.handle();
            let main_window = handler.get_webview_window("main").unwrap();
            let main_window_clone = main_window.clone();
            main_window_clone.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    let end_time = Local::now();
                    unsafe {
                        // 调用 save 函数保存数据
                        crate::use_time::save(
                            (format!("{}", START_TIME.unwrap().format("%Y-%m-%d %H:%M:%S")),
                             format!("{}", end_time.format("%Y-%m-%d %H:%M:%S"))
                            )
                        );
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}