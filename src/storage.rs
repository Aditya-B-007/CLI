use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Baseline {
    pub frequencies: HashMap<String, u32>,
}

pub fn save_baseline(freq: &HashMap<String, u32>) {
    let baseline = Baseline {
        frequencies: freq.clone(),
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