use std::collections::HashMap;
use crate::storage::Baseline;
pub type TimeBucket = String;
pub type LogLine = String;

pub struct Anomaly {
    pub line: String,
    pub baseline: u32,
    pub current: u32,
    pub ratio: f32,
}

pub fn detect_deviation(
    baseline: &Baseline,
    current: &HashMap<String, u32>,
) -> Vec<Anomaly> {

    let mut anomalies = Vec::new();

    for (line, &curr_count) in current.iter() {
        let base_count = baseline.frequencies.get(line).cloned().unwrap_or(0);
        let ratio = if base_count == 0 {
            curr_count as f32
        } else {
            curr_count as f32 / base_count as f32
        };
        if ratio > 3.0 && curr_count > 5 {
            anomalies.push(Anomaly {
                line: line.clone(),
                baseline: base_count,
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
        if !baseline.frequencies.contains_key(line) {
            novel.push(line.clone());
        }
    }

    novel
}

pub struct TemporalFrequency {
    pub buckets: HashMap<TimeBucket, HashMap<String, u32>>,
}

impl TemporalFrequency {
    pub fn new() -> Self {
        TemporalFrequency {
            buckets: HashMap::new(),
        }
    }

    pub fn add_log(&mut self, bucket: TimeBucket, line: LogLine) {
        let entry = self.buckets.entry(bucket).or_insert_with(HashMap::new);
        *entry.entry(line).or_insert(0) += 1;
    }
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
}//-->Naive, not good!!!