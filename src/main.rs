mod parser;
mod engine;
mod storage;
mod renderer;
mod shell;

use std::env;

use crate::engine::{
    build_temporal_frequency,
    detect_bursts,
    detect_periodicity,
    detect_stat_anomaly,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        renderer::print_banner();
        return;
    }

    let command = &args[1];
    let config = storage::load_config();
    match command.as_str() {
        "enable" => shell::enable(),
        "disable" => shell::disable(),
        "tune" => {
                if args.len() < 3 {
                    eprintln!("Usage: argus tune <level>");
                    return;
                }
                let level: f32 = args[2].parse().expect("Invalid sensitivity level. Please provide a number.");
                storage::update_config_sensitivity(level);
                println!("✅ Target sensitivity multiplier set to {}x", level);
            },
        "explain" => {
            let index: usize = args[2].parse().unwrap_or(0);
            let alerts = storage::load_latest_alerts(); // New storage function required
        
                if index > 0 && index <= alerts.len() {
                    renderer::print_explanation(&alerts[index - 1]);
                } else {
                    eprintln!("Invalid alert ID.");
                }
            },
        "learn" => {
            renderer::print_banner();

            let (logs_with_time, _) = parser::read_new_logs(&args[2], 0);
            let logs: Vec<String> = logs_with_time
                .iter() // Changed from into_iter to iter so we can borrow for both
                .map(|(_, line)| line.clone())
                .collect();
    
            let freq = engine::build_frequency(&logs, 1.0); // We'll update this function next
            storage::save_baseline(&freq);

    // ADD THESE LINES to build and save the Adaptive Baseline (Feature 3)
            let temporal_freq = engine::build_temporal_frequency(&logs_with_time);
            let adaptive_baseline = engine::build_adaptive_baseline(&temporal_freq);
            storage::save_adaptive_baseline(&adaptive_baseline);

            renderer::print_summary(&freq);
        }
        "check" => {
            let log_path = &args[2];
            let file_size = parser::get_file_size(log_path);
            let mut offset = storage::load_offset();
            let adaptive_baseline = storage::load_adaptive_baseline();
            if offset > file_size {
                storage::save_offset(file_size);
                offset = 0;
            }
            let (logs_with_time, new_offset) =
                parser::read_new_logs(log_path, offset);

            if logs_with_time.is_empty() {
                return; 
            }
            let logs: Vec<String> = logs_with_time
                .iter()
                .map(|(_, line)| line.clone())
                .collect();

            let log_count = logs.len();
            let sample_rate = if log_count > 5000 { // This is your pressure_threshold
                println!("⚠️ High volume detected ({} lines). Enabling 10% sampling mode.", log_count);
                0.1 
                } else {
                    1.0
                };
            let current = engine::build_frequency(&logs,sample_rate); // Update this function to accept sample_rate
            let baseline = storage::load_baseline();
            let novel = engine::detect_novelty(&baseline, &current);

            let z_threshold = config["z_threshold"].as_f64().unwrap_or(3.0) as f32;
            let deviation = engine::detect_deviation(&baseline, &current, z_threshold);

            let statistical = detect_stat_anomaly(&baseline, &current);

            let temporal = build_temporal_frequency(&logs_with_time);
            let adaptive_anomalies =
                engine::detect_adaptive_anomaly(&adaptive_baseline, &temporal);
            let adaptive_patterns: Vec<String> =
                adaptive_anomalies.iter().map(|a| a.line.clone()).collect();
            let bursts = detect_bursts(&temporal);
            let periodic = detect_periodicity(&temporal);

            let deviation_patterns: Vec<String> = deviation.iter().map(|a| a.line.clone()).collect();
            let statistical_patterns: Vec<String> = statistical.iter().map(|a| a.line.clone()).collect();
            let burst_patterns: Vec<String> = bursts.iter().map(|b| b.line.clone()).collect();

            let sensitivity_multiplier = storage::load_config_sensitivity();

            let alerts = engine::fuse_signals(
                &novel,
                &deviation_patterns,
                &statistical_patterns,
                &burst_patterns,
                &adaptive_patterns,
                sensitivity_multiplier,
                );
            storage::save_latest_alerts(&alerts);
            storage::save_offset(new_offset);
            renderer::print_novelty(&novel);
            renderer::print_anomalies(&deviation);
            renderer::print_anomalies(&statistical);
            renderer::print_bursts(&bursts);
            renderer::print_alerts(&alerts);
            renderer::print_periodicity(&periodic);
            }
            "banner" => renderer::print_banner(),

            _ => {
                eprintln!("Unknown command");
            }
        }
    }
