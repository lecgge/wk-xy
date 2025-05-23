use chrono::{DateTime, Datelike, NaiveDateTime, Utc, Local, TimeZone, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const USAGE_DATA_FILE: &str = "/home/service/usage_data.json";

#[derive(Serialize, Deserialize, Debug)]
struct UsageData {
    start_time: String,
    end_time: String,
}



pub fn calculate_usage(use_data: &Vec<(String, String)>) -> HashMap<String, i64> {
    let mut monthly_durations: HashMap<String, i64> = HashMap::new();
    let format_str = "%Y-%m-%d %H:%M:%S";

    for interval in use_data {
        let start_str = &interval.0;
        let end_str = &interval.1;

        // 解析开始时间
        let start = NaiveDateTime::parse_from_str(start_str, format_str)
            .map_err(|e| panic!("Invalid start time format: {}, error: {}", start_str, e))
            .and_then(|ndt| {
                Local.from_local_datetime(&ndt)
                    .single()
                    .ok_or_else(|| panic!("Ambiguous or invalid time for start: {}", start_str))
            })
            .unwrap_or_else(|_| panic!("Failed to parse start time: {}", start_str))
            .with_timezone(&Utc);

        // 解析结束时间
        let end = NaiveDateTime::parse_from_str(end_str, format_str)
            .map_err(|e| panic!("Invalid end time format: {}, error: {}", end_str, e))
            .and_then(|ndt| {
                Local.from_local_datetime(&ndt)
                    .single()
                    .ok_or_else(|| panic!("Ambiguous or invalid time for end: {}", end_str))
            })
            .unwrap_or_else(|_| panic!("Failed to parse end time: {}", end_str))
            .with_timezone(&Utc);

        let mut current = start;

        while current < end {
            let year = current.year();
            let month = current.month();

            let next_month = if month == 12 {
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap(),
                    Default::default(),
                )
            } else {
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap(),
                    Default::default(),
                )
            };

            let next_month_datetime = DateTime::<Utc>::from_utc(next_month, Utc);
            let month_end = std::cmp::min(next_month_datetime, end);

            let duration_seconds = (month_end - current).num_seconds();

            let key = format!("{}-{:02}", year, month);
            *monthly_durations.entry(key).or_insert(0) += duration_seconds;

            current = next_month_datetime;
        }
    }

    monthly_durations
}

fn format_seconds(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

pub fn load() -> Option<Vec<(String, String)>> {
    let path = Path::new(USAGE_DATA_FILE);
    if path.exists() {
        let data = fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
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
        let mut data_arry: Vec<(String, String)> = serde_json::from_str(&js)
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        data_arry.push(data);
        let json = serde_json::to_string_pretty(&data_arry)
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        fs::write(path, json).map_err(|e| e.to_string()).ok()
    } else {
        let mut data_arry: Vec<(String, String)> = Vec::new();
        data_arry.push(data);
        let json = serde_json::to_string_pretty(&data_arry)
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        fs::write(path, json).map_err(|e| e.to_string()).ok()
    }
}
