use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::engine::{self, Alert};

#[derive(Serialize, Deserialize)]
pub struct Offset {
    pub position: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stat {
    pub mean: f32,
    pub std_dev: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Baseline {
    pub stats: HashMap<String, Stat>,
}

#[derive(Serialize, Deserialize)]
pub struct AdaptiveBaseline {
    pub windows: HashMap<String, HashMap<String, Stat>>,
}

fn get_kautilya_dir() -> String {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    format!("{}/.kautilya", home)
}


pub fn load_config() -> serde_json::Value {
    let path = format!("{}/Config.json", get_kautilya_dir());

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str(&content) {
            return config;
        }
    }

    serde_json::json!({
        "z_threshold": 3.0,
        "burst_threshold": 10,
        "sensitivity_multiplier": 1.0
    })
}


pub fn save_baseline(freq: &HashMap<String, u32>) {
    let baseline = Baseline {
        stats: engine::build_stats(freq),
    };

    let json = serde_json::to_string_pretty(&baseline)
        .expect("Serialization failed");

    let dir = get_kautilya_dir();
    fs::create_dir_all(&dir).ok();

    let path = format!("{}/baseline.json", dir);

    fs::write(path, json)
        .expect("Failed to write baseline");
}

pub fn load_baseline() -> Baseline {
    let path = format!("{}/baseline.json", get_kautilya_dir());

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(baseline) = serde_json::from_str(&content) {
            return baseline;
        }
    }
    Baseline {
        stats: HashMap::new(),
    }
}

pub fn load_offset() -> u64 {
    let path = format!("{}/offset.json", get_kautilya_dir());

    if let Ok(data) = fs::read_to_string(path) {
        if let Ok(offset) = serde_json::from_str::<Offset>(&data) {
            return offset.position;
        }
    }

    0
}

pub fn save_offset(pos: u64) {
    let offset = Offset { position: pos };

    let json = serde_json::to_string(&offset)
        .expect("Offset serialization failed");

    let dir = get_kautilya_dir();
    fs::create_dir_all(&dir).ok();

    let path = format!("{}/offset.json", dir);

    fs::write(path, json)
        .expect("Failed to write offset");
}

pub fn save_latest_alerts(alerts: &Vec<Alert>) {
    let json = serde_json::to_string_pretty(alerts)
        .expect("Alerts serialization failed");

    let dir = get_kautilya_dir();
    fs::create_dir_all(&dir).ok();

    let path = format!("{}/alerts.json", dir);

    fs::write(path, json)
        .expect("Failed to write alerts");
}

pub fn load_latest_alerts() -> Vec<Alert> {
    let path = format!("{}/alerts.json", get_kautilya_dir());

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(alerts) = serde_json::from_str(&content) {
            return alerts;
        }
    }
    Vec::new()
}

pub fn save_adaptive_baseline(adaptive_baseline: &AdaptiveBaseline) {
    let json = serde_json::to_string_pretty(adaptive_baseline)
        .expect("Adaptive baseline serialization failed");

    let dir = get_kautilya_dir();
    fs::create_dir_all(&dir).ok();

    let path = format!("{}/adaptive_baseline.json", dir);

    fs::write(path, json)
        .expect("Failed to write adaptive baseline");
}

pub fn load_adaptive_baseline() -> AdaptiveBaseline {
    let path = format!("{}/adaptive_baseline.json", get_kautilya_dir());

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(adaptive_baseline) = serde_json::from_str(&content) {
            return adaptive_baseline;
        }
    }
    AdaptiveBaseline { windows: HashMap::new() }
}

pub fn load_config_sensitivity() -> f32 {
    let config = load_config();
    config["sensitivity_multiplier"].as_f64().unwrap_or(1.0) as f32
}

pub fn update_config_sensitivity(level: f32) {
    let mut config = load_config();
    config["sensitivity_multiplier"] = serde_json::to_value(level).unwrap();

    let dir = get_kautilya_dir();
    fs::create_dir_all(&dir).ok();

    let path = format!("{}/Config.json", dir);

    let json = serde_json::to_string_pretty(&config)
        .expect("Config serialization failed");

    fs::write(path, json)
        .expect("Failed to write config");
}