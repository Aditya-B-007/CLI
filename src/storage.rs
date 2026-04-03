use std::collections::HashMap;
use crate::engine;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Stat {
    pub mean: f32,
    pub std_dev: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub z_threshold: f32,
    pub burst_threshold: u32,
    pub window_size: usize,
}


#[derive(Serialize, Deserialize)]
pub struct Baseline {
    pub stats: HashMap<String, Stat>,
}

pub fn save_baseline(freq: &HashMap<String, u32>) {
    let baseline = Baseline {
        stats: engine::build_stats(freq),
    };

    let json = serde_json::to_string_pretty(&baseline)
        .expect("Serialization failed");

    fs::create_dir_all(".analyze").ok();

    fs::write(".analyze/baseline.json", json)
        .expect("Failed to write baseline");
}

pub fn load_baseline() -> Baseline {
    let content = fs::read_to_string(".analyze/baseline.json")
        .expect("Failed to read baseline");

    serde_json::from_str(&content)
        .expect("Deserialization failed")
}