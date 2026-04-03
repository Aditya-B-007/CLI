mod parser;
mod engine;
mod storage;
mod renderer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: analyze learn <logfile>");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
    "learn" => {
        let logs = parser::read_logs(file_path);
        let freq = engine::build_frequency(&logs);
        storage::save_baseline(&freq);
        renderer::print_summary(&freq);
    }
    "check" => {
    let logs = parser::read_logs_with_time(file_path);

    let temporal = engine::build_temporal_frequency(&logs);

    let flat_logs: Vec<String> = logs.iter().map(|(_, l)| l.clone()).collect();

    let current = engine::build_frequency(&flat_logs);
    let baseline = storage::load_baseline();

    let novel = engine::detect_novelty(&baseline, &current);
    let anomalies = engine::detect_deviation(&baseline, &current);

    let bursts = engine::detect_bursts(&temporal);
    let periodic = engine::detect_periodicity(&temporal);

    renderer::print_novelty(&novel);
    renderer::print_anomalies(&anomalies);
    renderer::print_bursts(&bursts);
    renderer::print_periodicity(&periodic);
}
    _ => {
        eprintln!("Unknown command: {}. Use 'learn' or 'check'.", command);
    }
    }
}