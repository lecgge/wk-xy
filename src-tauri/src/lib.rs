use std::thread::sleep;
use tokio::task;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod can_module;
mod can_matrix;
mod arxml_bean;


static mut CAN_MODULE: Option<can_module::CanModule> = None;

#[tauri::command]
async fn start(device: &str) -> Result<(), String>{
    // sleep(std::time::Duration::from_secs(10));
    println!("Hello, {}!", device);
    let mut can_matrix = can_matrix::CanMatrix::new();
    if device == "2712"{
        let _ = can_matrix.load_from_arxml("./resource/output.json");
    }else if device == "ZXD"{
        let _ = can_matrix.load_from_arxml("./resource/output.json");
    }else{
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start,stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}