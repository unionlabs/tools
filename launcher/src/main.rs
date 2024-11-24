#![allow(unused_variables)]

use std::{
    env,
    io::{BufRead, BufReader},
    process::Stdio,
    thread,
};

fn main() {
    let repository_absolute_path = env::var("REPOSITORY_ABSOLUTE_PATH").unwrap_or_else(|_| {
        eprintln!("REPOSITORY_ABSOLUTE_PATH is empty. Set it to the absolute path of the repository by running `export REPOSITORY_ABSOLUTE_PATH=/path/to/repository`");
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

    let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
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
                // `xdg-open $repo_vscode_url`
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

    let status = child
        .wait()
        .expect("Internal error, failed to wait on child");

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    let stdout = stdout_rx.into_iter().collect::<Vec<String>>().join("");
    let stderr = stderr_rx.into_iter().collect::<Vec<String>>().join("");
    // if stderr is not empty, print it and exit
    if !stderr.is_empty() {
        eprintln!("stderr: {}", stderr);
        std::process::exit(1);
    }
    println!("Hello, world!");
    println!("status: {}", status);
    println!("stdout: {}", stdout);
}
