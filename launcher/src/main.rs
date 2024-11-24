use std::{
    env,
    io::{BufRead, BufReader},
    process::Stdio,
    thread,
};

fn main() {
    let repository_absolute_path = env::var("REPOSITORY_ABSOLUTE_PATH").unwrap_or_else(|_| {
        eprintln!(
            "\n{}\n{}\n\n{}",
            colorize("REPOSITORY_ABSOLUTE_PATH is empty.", "red"),
            colorize(
                "Set it to the absolute path of the repository by running:",
                "yellow"
            ),
            colorize(
                "export REPOSITORY_ABSOLUTE_PATH=/path/to/repository",
                "green"
            )
        );
        std::process::exit(1);
    });

    let command_array = vec![
        "run",
        "github:nixos/nixpkgs/nixpkgs-unstable#openvscode-server",
        "--",
        "--port=6699",
        "--host=127.0.0.1",
        "--update-extensions",
        "--disable-telemetry",
        "--accept-server-license-terms",
        "--start-server",
    ];
    let mut command = std::process::Command::new("nix");
    command.args(command_array);
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdout = child
        .stdout
        .take()
        .expect("Internal error, could not take stdout");
    let child_stderr = child
        .stderr
        .take()
        .expect("Internal error, could not take stderr");

    let (stdout_tx, _) = std::sync::mpsc::channel();
    let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

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
                "http://127.0.0.1:6699?tkn={}&folder={}",
                token, repository_absolute_path
            );

            println!("");
            println!("{}", repo_vscode_url);

            // check if user passed the `--open` flag
            let args: Vec<String> = env::args().collect();
            if args.len() > 1 && args[1] == "--open" {
                println!("Opening the vscode url in the default browser...");
                // open the vscode url in the default browser
                let _ = std::process::Command::new("open")
                    .arg(repo_vscode_url)
                    .output()
                    .expect("Failed to open the vscode url in the default browser");
            }

            stdout_tx.send(line).unwrap();
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

    let stderr = stderr_rx.into_iter().collect::<Vec<String>>().join("");
    // if stderr is not empty, print it and exit
    if !stderr.is_empty() {
        eprintln!("stderr: {}", stderr);
        std::process::exit(1);
    }
}

fn colorize(text: &str, color: &str) -> String {
    let color_code = match color.to_lowercase().as_str() {
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        _ => "0", // default to no color
    };
    format!("\x1b[1;{}m{}\x1b[0m", color_code, text)
}
