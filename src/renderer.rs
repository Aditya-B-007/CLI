use std::collections::HashMap;
use crate::engine::Anomaly;
use crate::engine::Burst;
use crate::engine::Alert;
use std::{thread, time};

pub fn render_stream(cmd: &str, alert: Option<Alert>) {
    match alert {
        Some(a) => {
            println!("🚨 {} | score: {:.2} | {:?}", cmd, a.score, a.reasons);
        }
        None => {
            println!("✔ {}", cmd);
        }
    }
}

pub fn animate_line() {
    let frames = ["⠁", "⠂", "⠄", "⠂"];

    for frame in frames {
        print!("\r{} analyzing...", frame);
        use std::io::Write;
        std::io::stdout().flush().unwrap();

        thread::sleep(time::Duration::from_millis(80));
    }

    print!("\r                 \r"); // clear line
}
pub fn print_alerts(alerts: &Vec<Alert>) {
    if alerts.is_empty() { return; }
    println!("🚨 Detected alerts:\n");
    for alert in alerts {
        println!(
            "🚨 {} | score: {:.1} | reasons: {:?}",
            alert.pattern, alert.score, alert.reasons
        );
    }
    println!(); // Add a newline for better separation
}
pub fn print_summary(freq: &HashMap<String, u32>) {
    println!("Top log patterns:\n");

    let mut items: Vec<_> = freq.iter().collect();

    items.sort_by(|a, b| b.1.cmp(a.1));

    for (line, count) in items.iter().take(10) {
        println!("[{}] {}", count, line);
    }
}


pub fn print_anomalies(anomalies: &Vec<Anomaly>) {
    if anomalies.is_empty() {
        // If no anomalies, print nothing. The caller (main.rs) should handle a consolidated "No anomalies detected." message if all anomaly types are empty.
        return;
    }

    println!("⚠️ Detected anomalies:\n");

    for a in anomalies {
        println!(
            "⚠️ {} [{} → {}] ({:.2}x)",
            a.line,
            a.baseline,
            a.current,
            a.ratio,
        );
    }
}


pub fn print_novelty(novel: &Vec<String>) {
    if novel.is_empty() {
        return;
    }

    println!("🚨 New patterns detected:\n");

    for line in novel {
        println!("🚨 {}", line);
    }

    println!();
    println!("────────────────────────────────────────────────────────\n");
}

pub fn print_bursts(bursts: &Vec<Burst>) {
    if bursts.is_empty() {
        return;
    }

    println!("⚡ Burst activity detected:\n");

    for b in bursts {
        println!(
            "⚡ [{}] {} → {} occurrences",
            b.bucket, b.line, b.count
        );
    }

    println!();
}

pub fn print_periodicity(periodic: &Vec<String>) {
    if periodic.is_empty() {
        return;
    }

    println!("⏱️ Periodic patterns detected:\n");

    for line in periodic {
        println!("⏱️ {}", line);
    }

    println!();
}

pub fn print_banner() {
    let saffron = "\x1b[38;5;208m";
    let dim = "\x1b[2m"; 
    let bold = "\x1b[1m";        
    let reset = "\x1b[0m";
    let banner = r#"
      ___          _  __   _   _   _  _____  ___  _   __   __  _    
     / _ \  _ _   | |/ /  /_\ | | | ||_   _||_ _|| |  \ \ / / /_\   
    | (_) |(_|_)  | ' <  / _ \| |_| |  | |   | | | |__ \ V / / _ \  
     \___/ / /    |_|\_\/_/ \_\\___/   |_|  |___||____| |_| /_/ \_\ 
      | | / /     
      |_|/_/      
    "#;

    println!("{}{}{}", bold, saffron, banner);
    println!(
        "{}{}{} {}Kautilya{} {}Strategic Intelligence System Online{} {}", 
        saffron, bold, reset, bold, reset, dim, reset, reset
    );
    println!();
}

pub fn print_explanation(alert: &Alert) {
    println!("\n--- Alert Explanation ---");
    println!("Pattern: {}", alert.pattern);
    println!("Score: {:.1}", alert.score);
    println!("Reasons:");
    for reason in &alert.reasons {
        println!("  - {}", reason);
    }
}