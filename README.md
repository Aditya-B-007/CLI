# 🛡️ Kautilya
**Strategic Log Intelligence & Anomaly Detection System**

Kautilya is a terminal-native, lightweight security and monitoring tool built in Rust. It learns your system's "normal" behavior and uses deterministic statistical models to flag unusual patterns, rare events, and potential threats in real-time.

Unlike traditional logging tools that are noisy and complex, Kautilya focuses exclusively on **what is unusual**, providing high-context alerts with near-zero latency and zero reliance on external AI.

---

## ✨ Key Features

* **Self-Learning Baselines**: Learns from historical logs to establish a mathematical fingerprint of "normal" system activity.
* **Multi-Signal Detection Engine**: Fuses five distinct signals to identify anomalies:
    * **Novelty**: Flags never-before-seen patterns.
    * **Statistical Deviation**: Detects frequency spikes using Z-score analysis.
    * **Temporal Bursts**: Identifies rapid-fire command execution.
    * **Adaptive Windows**: Understands that "normal" at 10 AM is different from 2 AM.
    * **Sequence Intelligence**: Recognizes unusual command chains.
* **Shell Integration**: Hooks directly into `Zsh` and `Bash` to monitor commands as you type.
* **Privacy-First & Local**: All analysis happens on your machine. No data ever leaves your local environment.
* **Explainable Alerts**: Don't just get alerted; understand *why* a pattern was flagged with the `explain` command.

---

## 🚀 Getting Started

### Installation
Ensure you have the Rust toolchain installed, then clone and build the project:

```bash
git clone https://github.com/aditya-b-007/Kautilya.git
cd Kautilya
cargo build --release
```

### Setup
Enable the shell hook to start monitoring your terminal activity:

```bash
./target/release/kautilya enable
source ~/.zshrc  # or ~/.bashrc
```

### Initial Training
Feed Kautilya your existing logs to help it learn your baseline:

```bash
kautilya learn /path/to/your/history_logs.txt
```

---

## 🛠️ Usage

| Command | Description |
| :--- | :--- |
| `kautilya stream` | Start live monitoring of incoming log streams. |
| `kautilya check <file>` | Run a one-time anomaly scan on a specific log file. |
| `kautilya explain <id>` | Get a detailed breakdown of a specific alert's score and reasons. |
| `kautilya tune <level>` | Adjust the sensitivity multiplier (e.g., `1.5` for higher sensitivity). |
| `kautilya disable` | Safely remove hooks from your shell configuration. |

---

## ⚙️ Configuration
The system behavior can be customized via `~/.kautilya/Config.json`:

```json
{
  "z_threshold": 3.0,
  "burst_threshold": 10,
  "sensitivity_multiplier": 1.0
}
```

---

## 🏗️ Architecture
Kautilya is built with modularity and performance in mind:
* **`engine.rs`**: The brain; handles statistical modeling and signal fusion.
* **`parser.rs`**: Normalizes raw logs into generic patterns (e.g., masking timestamps and IDs).
* **`shell.rs`**: Manages terminal hooks and environment integration.
* **`renderer.rs`**: Handles terminal UI, animations, and alert formatting.

---

## 📄 License
Distributed under the **Apache License, Version 2.0**. See `LICENSE` for more information.
