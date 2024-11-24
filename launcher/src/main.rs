use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::Stdio,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
struct FlagConfig<'a> {
    name: &'a str,
    default: Option<&'a str>,
    required: bool,
}

fn main() {
    // check if `--help` flag is passed
    if std::env::args().any(|arg| arg == "--help") {
        println!(
            "\n{0} {1} {2}",
            text_effect("Usage:", "bold"),
            text_effect("launcher", "bold"),
            text_effect("[flags]", "bold")
        );
        println!("\n{0}", text_effect("Flags:", "bold"));
        println!(
            "  {0} {1} {2}",
            text_effect("--port=<port>", "green"),
            text_effect(" Port on which the vscode server will run", "italic"),
            text_effect("    [default: 6699]", "yellow")
        );
        println!(
            "  {0} {1} {2}",
            text_effect("--path=<path>", "green"),
            text_effect(" Repository to launch in vscode", "italic"),
            text_effect("              [default: pwd]", "yellow")
        );
        println!(
            "  {0} {1} {2}",
            text_effect("--host=<host>", "green"),
            text_effect(" Host on which the vscode server will run", "italic"),
            text_effect("    [default: 127.0.0.1]", "yellow")
        );
        println!(
            "  {0} {1} {2}",
            text_effect("--open", "green"),
            text_effect(
                "        Launch vscode server in the default browser",
                "italic"
            ),
            text_effect(" [default: false]", "yellow")
        );
        println!(
            "  {0} {1}",
            text_effect("--help", "green"),
            text_effect("        Print this help message", "italic")
        );
        println!("\n{0}", text_effect("Example:", "bold"));
        println!(
            "  {0} {1}",
            text_effect("launcher", "cyan"),
            text_effect(
                "--path=/path/to/repo --port=6699 --host=localhost --open",
                "blue"
            )
        );
        std::process::exit(0);
    }

    // parse the cli args
    let flag_configs = &[
        FlagConfig {
            name: "--path",
            default: Some("."),
            required: false,
        },
        FlagConfig {
            name: "--port",
            default: Some("6699"),
            required: false,
        },
        FlagConfig {
            name: "--host",
            default: Some("127.0.0.1"),
            required: false,
        },
        FlagConfig {
            name: "--open",
            default: Some("false"),
            required: false,
        },
    ];

    let (open_arg, path_arg, port_arg, host_arg) = match parse_args(flag_configs) {
        Ok((parsed_flags, _other_args)) => (
            parsed_flags.get("--open").unwrap().clone(),
            parsed_flags.get("--path").unwrap().clone(),
            parsed_flags.get("--port").unwrap().clone(),
            parsed_flags.get("--host").unwrap().clone(),
        ),
        Err(err) => {
            eprintln!("\n{}", text_effect(&err, "red"));
            std::process::exit(1);
        }
    };

    let mut command = std::process::Command::new("nix");
    command
        .arg("run")
        .arg("github:nixos/nixpkgs/nixpkgs-unstable#openvscode-server")
        .arg("--")
        .arg(format!("--port={}", port_arg))
        .arg(format!("--host={}", host_arg))
        .args(&[
            "--update-extensions",
            "--disable-telemetry",
            "--accept-server-license-terms",
            "--start-server",
        ]);

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let child_stdout = child
        .stdout
        .take()
        .expect("Internal error, could not take stdout");
    let child_stderr = child
        .stderr
        .take()
        .expect("Internal error, could not take stderr");

    let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

    let wait_group = Arc::new(Mutex::new(()));

    // This is where thread spawning begins
    let stdout_thread = thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            // if line doesn't start with "Web UI available at",continue
            if line.starts_with("Web UI available at") == false {
                continue;
            }
            // extracting the `tkn` part
            let token = line.split("tkn=").collect::<Vec<&str>>()[1];
            let repo_vscode_url = format!(
                "http://{}:{}?tkn={}&folder={}",
                host_arg, port_arg, token, path_arg,
            );

            println!("\n{}", repo_vscode_url);

            if open_arg == "true" {
                let _ = std::process::Command::new("xdg-open")
                    .arg(repo_vscode_url)
                    .spawn()
                    .expect("Failed to open browser");
            }
        }
    });

    let stderr_thread = thread::spawn(move || {
        let stderr_lines = BufReader::new(child_stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            eprintln!("{}", line);
            stderr_tx.send(line).unwrap();
        }
    });
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    drop(wait_group); // block until both threads have finished

    let stderr = stderr_rx.iter().collect::<Vec<String>>().join("\n");
    if !stderr.is_empty() {
        eprintln!("stderr: {}", stderr);
        std::process::exit(1);
    }
}

fn parse_args(
    flag_configs: &[FlagConfig],
) -> Result<(HashMap<String, String>, Vec<String>), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut parsed_flags = HashMap::new();
    let mut other_args = Vec::new();

    // Initialize with defaults and check for required flags
    for config in flag_configs {
        if let Some(default) = config.default {
            parsed_flags.insert(config.name.to_string(), default.to_string());
        }
    }

    for arg in args {
        let mut matched = false;
        for config in flag_configs {
            if arg.starts_with(&format!("{}=", config.name)) {
                let value = arg
                    .trim_start_matches(&format!("{}=", config.name))
                    .to_string();
                parsed_flags.insert(config.name.to_string(), value);
                matched = true;
                break;
            }
        }
        if !matched {
            other_args.push(arg);
        }
    }

    // Check if all required flags are provided
    for config in flag_configs {
        if config.required && !parsed_flags.contains_key(config.name) {
            return Err(format!("Required flag '{}' is missing", config.name));
        }
    }

    Ok((parsed_flags, other_args))
}

fn text_effect(text: &str, effect: &str) -> String {
    let effect_code = match effect.to_lowercase().as_str() {
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        "bold" => "1",
        "italic" => "3",
        "underline" => "4",
        "blink" => "5",
        "inverse" => "7",
        "hidden" => "8",
        "strikethrough" => "9",
        _ => "0", // default to no color
    };
    format!("\x1b[1;{}m{}\x1b[0m", effect_code, text)
}
