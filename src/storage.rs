use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::engine;

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


pub fn load_config() -> serde_json::Value {
    let path = format!("{}/Config.json", get_argus_dir());

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str(&content) {
            return config;
        }
    }

    serde_json::json!({
        "z_threshold": 3.0,
        "burst_threshold": 10
    })
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