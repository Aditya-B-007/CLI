use std::env;
use std::fs;
use std::path::Path;

const START: &str = "# >>> kautilya hook >>>";
const END: &str = "# <<< kautilya hook <<<";

pub fn enable() {
    let rc_path = get_rc_path();

    let mut content = if Path::new(&rc_path).exists() {
        fs::read_to_string(&rc_path).unwrap_or_default()
    } else {
        // Minimal safe default
        String::from("# kautilya generated shell config\n")
    };

    // Prevent duplication
    if content.contains(START) {
        println!("kautilya already enabled.");
        return;
    }

    let hook = build_hook();

    content.push_str("\n");
    content.push_str(&hook);

    fs::write(&rc_path, content).expect("Failed to write rc file");

    println!("kautilya enabled.");
    println!("Run: source {}", rc_path);
}

pub fn disable() {
    let rc_path = get_rc_path();

    if !Path::new(&rc_path).exists() {
        println!("No shell config found.");
        return;
    }

    let content = fs::read_to_string(&rc_path).unwrap_or_default();

    let cleaned = remove_block(&content);

    fs::write(&rc_path, cleaned).expect("Failed to update rc file");

    println!("kautilya disabled.");
}

fn get_rc_path() -> String {
    let home = env::var("HOME").unwrap();
    let shell = env::var("SHELL").unwrap_or_default();

    if shell.contains("zsh") {
        format!("{}/.zshrc", home)
    } else {
        format!("{}/.bashrc", home)
    }
}

fn remove_block(content: &str) -> String {
    let mut result = Vec::new();
    let mut inside = false;

    for line in content.lines() {
        if line.contains(START) {
            inside = true;
            continue;
        }
        if line.contains(END) {
            inside = false;
            continue;
        }
        if !inside {
            result.push(line);
        }
    }

    result.join("\n")
}

fn build_hook() -> String {
    format!(
r#"{start}

kautilya_BIN="kautilya"
kautilya_LOGS="$HOME/.kautilya/logs.txt"

_kautilya_init() {{
    if [ -z "$kautilya_INITIALIZED" ]; then
        $kautilya_BIN banner
        export kautilya_INITIALIZED=1
    fi
}}

_kautilya_run() {{
    if [ -n "$kautilya_RUNNING" ]; then
        return
    fi

    export kautilya_RUNNING=1

    cmd=$(history 1 | sed 's/^ *[0-9]* *//')

    kautilya send "$cmd"

    unset kautilya_RUNNING
    }}

if [ -n "$ZSH_VERSION" ]; then
    precmd_functions+=(_kautilya_init)
    precmd_functions+=(_kautilya_run)
elif [ -n "$BASH_VERSION" ]; then
    PROMPT_COMMAND="_kautilya_init; _kautilya_run"
fi

{end}
"#,
        start = START,
        end = END
    )
}