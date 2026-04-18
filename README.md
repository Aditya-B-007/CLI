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

### 👤 Daily Usage Guide

Kautilya offers two modes of operation depending on how much feedback you want.

#### Mode 1: The "Set and Forget" Daemon (Background)
Best if you want to keep your terminal clean and only see alerts when something is actually suspicious.

1.  **Start the Daemon:** In one terminal window, run:
    ```bash
    kautilya pipe-listen
    ```
2.  **Work as usual:** Open a new tab and run your daily commands.
3.  **Silent Analysis:** Kautilya monitors everything in the background. It only alerts you (via the Daemon window) if it detects an anomaly (e.g., `🚨 cd` or a novel command).

#### Mode 2: The "Live Stream" (Interactive)
Best if you want to see exactly what Kautilya thinks of every single command you type in real-time.

1.  **Start the Stream:** In your active terminal, run:
    ```bash
    kautilya stream
    ```
2.  **Immediate Feedback:** Now, as you type commands, you will get instant visual confirmation in your current window:
    * `✔ ls` (Normal activity)
    * `🚨 cd | score: 6.00 | ["novel", "sequence"]` (Anomaly detected)

---

### ⚙️ How it Works (Under the Hood)


Kautilya uses a decoupled architecture to ensure your terminal remains fast and responsive.

1.  **The Sensor (Shell Hook):** A lightweight hook in your shell configuration automatically captures every command you type.
2.  **The Transmission:** The hook sends your command string to the background Daemon over a local TCP socket (`127.0.0.1:7878`).
3.  **The Brain (Engine):** The Daemon receives the command, normalizes it (stripping IDs and timestamps), and runs it through a multi-signal engine to calculate a "novelty" and "statistical" score.
4.  **The Output:** The system compares your command against your historical baseline. If the score exceeds your configured threshold, it triggers an alert.

> **Note:** Because the shell hook "fires and forgets" the command, Kautilya adds **zero latency** to your terminal experience—your prompt returns immediately, regardless of how complex the analysis is.

## 📄 License
Distributed under the **Apache License, Version 2.0**. See `LICENSE` for more information.
