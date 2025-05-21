use std::thread::sleep;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod can_module;
mod can_matrix;
mod arxml_bean;


static mut CAN_MODULE: Option<can_module::CanModule> = None;

#[tauri::command]
#[tokio::main]
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

    can_module::start(can_matrix,"can0").await.expect("发送消息报错");
    Ok(())
}

#[tauri::command]
fn stop() {
    unsafe {
        if let Some(can_module) = CAN_MODULE.as_mut() {
            can_module.drop();
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