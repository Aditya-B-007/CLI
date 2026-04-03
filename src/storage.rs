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

fn get_argus_dir() -> String {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    format!("{}/.argus", home)
}

pub fn save_baseline(freq: &HashMap<String, u32>) {
    let baseline = Baseline {
        stats: engine::build_stats(freq),
    };

    let json = serde_json::to_string_pretty(&baseline)
        .expect("Serialization failed");

    let dir = get_argus_dir();
    fs::create_dir_all(&dir).ok();

    let path = format!("{}/baseline.json", dir);

    fs::write(path, json)
        .expect("Failed to write baseline");
}

pub fn update_config_sensitivity(level: f32) {
    let dir = get_argus_dir();
    let path = format!("{}/Config.json", dir);
    let mut config: serde_json::Value = if let Ok(content) = fs::read_to_string(&path) {
        serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({
            "z_threshold": 3.0,
            "burst_threshold": 10,
            "window_size": 50
        }))
    } else {
        serde_json::json!({
            "z_threshold": 3.0,
            "burst_threshold": 10,
            "window_size": 50
        })
    };
    config["sensitivity_multiplier"] = serde_json::json!(level);

    // 3. Persist the changes
    let json = serde_json::to_string_pretty(&config).expect("Config serialization failed");
    fs::write(path, json).expect("Failed to write Config.json");
}

pub fn load_baseline() -> Baseline {
    let path = format!("{}/baseline.json", get_argus_dir());

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
    let path = format!("{}/offset.json", get_argus_dir());

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

    let dir = get_argus_dir();
    fs::create_dir_all(&dir).ok();

    let path = format!("{}/offset.json", dir);

    fs::write(path, json)
        .expect("Failed to write offset");
}

pub fn load_config_sensitivity() -> f32 {
    let dir = get_argus_dir();
    let path = format!("{}/Config.json", dir);
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(multiplier) = config["sensitivity_multiplier"].as_f64() {
                return multiplier as f32;
            }
        }
    }
    1.0
}

#[derive(Serialize, Deserialize)]
pub struct AdaptiveBaseline {
    pub windows: HashMap<String, HashMap<String, Stat>>, 
}

pub fn save_adaptive_baseline(adaptive_baseline: &AdaptiveBaseline) {
    let json = serde_json::to_string_pretty(adaptive_baseline)
        .expect("Adaptive baseline serialization failed");

    let dir = get_argus_dir();
    fs::create_dir_all(&dir).ok(); // Ensure directory exists

    let path = format!("{}/adaptive_baseline.json", dir);

    fs::write(path, json)
        .expect("Failed to write adaptive baseline");
}

pub fn load_adaptive_baseline() -> AdaptiveBaseline {
    let path = format!("{}/adaptive_baseline.json", get_argus_dir());

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(adaptive_baseline) = serde_json::from_str(&content) {
            return adaptive_baseline;
        }
    }
    // Return a default empty AdaptiveBaseline if file not found or deserialization fails
    AdaptiveBaseline {
        windows: HashMap::new(),
    }
}
pub fn save_latest_alerts(alerts: &Vec<Alert>) {
    let json = serde_json::to_string(alerts).expect("Alert serialization failed");
    let path = format!("{}/latest_alerts.json", get_argus_dir());
    fs::write(path, json).expect("Failed to write latest alerts");
}

pub fn load_latest_alerts() -> Vec<Alert> {
    let path = format!("{}/latest_alerts.json", get_argus_dir());
    let content = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_else(|_| vec![])
}