use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const USAGE_DATA_FILE: &str = "/home/service/usage_data.json";

#[derive(Serialize,Deserialize,Debug)]
struct UsageData {
    start_time: String,
    end_time: String,
}

pub fn load() -> Option<Vec<(String, String)>> {
    let path = Path::new(USAGE_DATA_FILE);
    if path.exists() {
        let data = fs::read_to_string(path).map_err(|e| e.to_string()).ok().unwrap();
        serde_json::from_str(&data).map_err(|e| e.to_string()).ok()
    } else {
        None
    }
}

pub fn save(data: (String, String)) -> Option<()> {
    let path = Path::new(USAGE_DATA_FILE);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok()?;
    }
    let json_string = fs::read_to_string(&path).map_err(|e| e.to_string()).ok();
    if let Some(js) = json_string {
        let mut data_arry :  Vec<(String,String)> = serde_json::from_str(&js).map_err(|e| e.to_string()).ok().unwrap();
        data_arry.push(data);
        let json = serde_json::to_string_pretty(&data_arry).map_err(|e| e.to_string()).ok().unwrap();
        fs::write(path, json).map_err(|e| e.to_string()).ok()
    } else {
        let mut data_arry :  Vec<(String,String)> = Vec::new();
        data_arry.push(data);
        let json = serde_json::to_string_pretty(&data_arry).map_err(|e| e.to_string()).ok().unwrap();
        fs::write(path, json).map_err(|e| e.to_string()).ok()
    }
    
}