use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use regex::Regex;
pub fn get_file_size(path: &str) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

pub fn read_new_logs(path: &str, offset: u64) -> (Vec<(String, String)>, u64) {
    let file = File::open(path).expect("Failed to open log file");
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::Start(offset)).unwrap();

    let mut logs = Vec::new();
    let mut current_pos = offset;
    let mut line = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 { break; }
        current_pos += bytes_read as u64;
        let bucket = extract_time_bucket(&line);
        let processed = process_line(&line);
        logs.push((bucket, processed));
        line.clear();
    }

    (logs, current_pos)
}

pub fn process_line(line: &str) -> String {
    let normalized = normalize(line);
    tokenize(&normalized)
}

pub fn tokenize(line: &str) -> String {
    line
        .split_whitespace()
        .map(|token| normalize_token(token))
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_token(token: &str) -> String {
    if token.chars().all(|c| c.is_numeric()) {
        "*".to_string()
    } else {
        token.to_string()
    }
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

pub fn extract_time_bucket(line: &str) -> String {
    let re = Regex::new(r"\b\d{2}:\d{2}:\d{2}\b").unwrap();

    if let Some(mat) = re.find(line) {
        let time = mat.as_str();
        return time[0..5].to_string(); 
    }

    "unknown".to_string()
}