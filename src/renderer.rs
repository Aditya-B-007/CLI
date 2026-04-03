use std::collections::HashMap;
use crate::engine::Anomaly;
use crate::engine::Burst;
use crate::engine::Alert;

pub fn print_alerts(alerts: &Vec<Alert>) {
    for alert in alerts {
        println!(
            "🚨 {} | score: {:.1} | reasons: {:?}",
            alert.pattern, alert.score, alert.reasons
        );
    }
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
        println!("No anomalies detected.");
        return;
    }

    println!("⚠️ Detected anomalies:\n");

    for a in anomalies {
        println!(
            "⚠️ [{} → {}] ({:.2}x)\n{}",
            a.baseline,
            a.current,
            a.ratio,
            a.line
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
    let red = "\x1b[38;5;196m";  
    let dim = "\x1b[2m"; 
    let bold = "\x1b[1m";        
    let reset = "\x1b[0m";

    let banner = r#"
     .---.        █████╗ ██████╗  ██████╗ ██╗   ██╗███████╗
    /  _  \__    ██╔══██╗██╔══██╗██╔════╝ ██║   ██║██╔════╝
    | ( )  __\   ███████║██████╔╝██║  ███╗██║   ██║███████╗
    \  -  /      ██╔══██║██╔══██╗██║   ██║██║   ██║╚════██║
     \___/___    ██║  ██║██║  ██║╚██████╔╝╚██████╔╝███████║
      /|   /|    ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚══════╝
    "#;

    // Printing the banner in bold red
    println!("{}{}{}", bold, red, banner);
    
    // Metadata line with a mix of styles for a professional feel
    println!(
        "{}{}{} {}Argus{} {}Log Intelligence Engine Initialized{} {}", 
        red, bold, reset, bold, reset, dim, reset, reset
    );
    println!();
}