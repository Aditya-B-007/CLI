use std::fs;
use regex::Regex;
use chrono::{DateTime, Local};
use regex::Regex;


pub fn read_logs(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path)
        .expect("Failed to read log file");

    content
        .lines()
        .map(|line| normalize(line))
        .collect()
}

fn normalize(line: &str) -> String {
    let mut s = line.to_string();
    let re_time = Regex::new(r"\b\d{2}:\d{2}:\d{2}\b").unwrap();
    s = re_time.replace_all(&s, "*").to_string();
    let re_date = Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").unwrap();
    s = re_date.replace_all(&s, "*").to_string();
    let re_num = Regex::new(r"\b\d+\b").unwrap();
    s = re_num.replace_all(&s, "*").to_string();
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn read_logs_with_time(path: &str) -> Vec<(String, String)> {
    let content = fs::read_to_string(path)
        .expect("Failed to read log file");

    content
        .lines()
        .map(|line| {
            let bucket = extract_time_bucket(line);
            let normalized = normalize(line);
            (bucket, normalized)
        })
        .collect()
}

fn extract_time_bucket(line: &str) -> String {
    let re = Regex::new(r"\b\d{2}:\d{2}:\d{2}\b").unwrap();

    if let Some(mat) = re.find(line) {
        let time = mat.as_str();
        return time[0..5].to_string();
    }

    "unknown".to_string()
}