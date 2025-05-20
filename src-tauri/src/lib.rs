// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod can_module;
mod can_matrix;
mod arxml_bean;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tokio::main]
pub async fn start(){
    let mut can_matrix = can_matrix::CanMatrix::new();
    can_matrix.load_from_arxml("./resource/output.json");
    //读取json文件
    // let data = fs::read_to_string("./resource/output.json")?;
    // // println!("Parsed JSON:\n{:#?}", data);
    // let v: HashMap<String,Vec<Frame>> = serde_json::from_str(&data)?;
    // println!("{:?}", can_matrix);
    can_module::start(can_matrix).await.expect("发送消息报错");
}
