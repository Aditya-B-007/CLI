use std::collections::HashMap;
use crate::storage::{Baseline, Stat};
pub type TimeBucket = String;

pub fn fuse_signals(
    novelty: &Vec<String>,
    deviation: &Vec<String>,
    statistical: &Vec<String>,
    bursts: &Vec<String>,
) -> Vec<Alert> {

    let mut map: HashMap<String, Alert> = HashMap::new();

    // Helper closure
    let mut add_signal = |pattern: &String, weight: f32, reason: &str| {
        let entry = map.entry(pattern.clone()).or_insert(Alert {
            pattern: pattern.clone(),
            score: 0.0,
            reasons: vec![],
        });

        entry.score += weight;
        entry.reasons.push(reason.to_string());
    };

    for p in novelty {
        add_signal(p, 3.0, "novel");
    }

    for p in deviation {
        add_signal(p, 2.0, "deviation");
    }

    for p in statistical {
        add_signal(p, 2.5, "statistical");
    }

    for p in bursts {
        add_signal(p, 2.0, "burst");
    }

    let mut alerts: Vec<Alert> = map.into_values().collect();

    // Sort by score descending
    alerts.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    alerts
}
#[derive(Debug)]
pub struct Alert {
    pub pattern: String,
    pub score: f32,
    pub reasons: Vec<String>,
}
pub struct Anomaly {
    pub line: String,
    pub baseline: f32,
    pub current: u32,
    pub ratio: f32,
}
pub fn build_stats(freq: &HashMap<String, u32>) -> HashMap<String, Stat> {
    let mut stats = HashMap::new();

    for (line, &count) in freq.iter() {
        let mean = count as f32; 
        let std_dev = 0.0; 
        stats.insert(line.clone(), Stat { mean, std_dev });
    }

    stats
}
pub fn detect_deviation(
    baseline: &Baseline,
    current: &HashMap<String, u32>,
) -> Vec<Anomaly> {

    let mut anomalies = Vec::new();

    for (line, &curr_count) in current.iter() {
        let base_count = baseline.stats.get(line).cloned().unwrap_or(Stat { mean: 0.0, std_dev: 0.0 });
        let ratio = if base_count.mean == 0.0 {
            curr_count as f32
        } else {
            curr_count as f32 / base_count.mean
        };
        if ratio > 3.0 && curr_count > 5 {
            anomalies.push(Anomaly {
                line: line.clone(),
                baseline: base_count.mean,
                current: curr_count,
                ratio,
            });
        }
    }

    anomalies
}

pub fn detect_novelty(
    baseline: &Baseline,
    current: &HashMap<String, u32>,
) -> Vec<String> {
    let mut novel = Vec::new();

    for (line, _) in current.iter() {
        if !baseline.stats.contains_key(line) {
            novel.push(line.clone());
        }
    }

    novel
}

pub struct TemporalFrequency {
    pub buckets: HashMap<TimeBucket, HashMap<String, u32>>,
}

pub fn build_temporal_frequency(
    logs: &Vec<(String, String)>
) -> TemporalFrequency {

    let mut buckets: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for (bucket, line) in logs {
        let entry = buckets
            .entry(bucket.clone())
            .or_insert_with(HashMap::new);

        *entry.entry(line.clone()).or_insert(0) += 1;
    }

    TemporalFrequency { buckets }
}

pub struct Burst {
    pub bucket: String,
    pub line: String,
    pub count: u32,
}

pub fn detect_bursts(
    temporal: &TemporalFrequency
) -> Vec<Burst> {

    let mut bursts = Vec::new();

    for (bucket, freq_map) in &temporal.buckets {
        for (line, &count) in freq_map {
            if count > 10 { 
                bursts.push(Burst {
                    bucket: bucket.clone(),
                    line: line.clone(),
                    count,
                });
            }
        }
    }

    bursts
}

pub fn detect_periodicity(
    temporal: &TemporalFrequency
) -> Vec<String> {

    let mut occurrences: HashMap<String, Vec<String>> = HashMap::new();

    for (bucket, freq_map) in &temporal.buckets {
        for (line, _) in freq_map {
            occurrences
                .entry(line.clone())
                .or_insert_with(Vec::new)
                .push(bucket.clone());
        }
    }

    let mut periodic = Vec::new();

    for (line, buckets) in occurrences {
        if buckets.len() >= 3 {
            periodic.push(line);
        }
    }
    periodic
}

pub fn build_frequency(logs: &Vec<String>) -> HashMap<String, u32> {
    let mut freq = HashMap::new();

    for line in logs {
        *freq.entry(line.clone()).or_insert(0) += 1;
    }

    freq
}

pub fn detect_stat_anomaly(
    baseline: &Baseline,
    current: &HashMap<String, u32>,
) -> Vec<Anomaly> {

    let mut anomalies = Vec::new();

    for (line, &curr) in current {
        if let Some(stat) = baseline.stats.get(line) {
            let z = (curr as f32 - stat.mean) / stat.std_dev.max(1.0);

            if z > 3.0 {
                anomalies.push(Anomaly {
                    line: line.clone(),
                    baseline: stat.mean,
                    current: curr,
                    ratio: z,
                });
            }
        }
    }

    anomalies
}