use std::{
    io::{BufRead, BufReader},
    process::Stdio,
    sync::{Arc, Mutex},
    thread,
};

use clap::{Command, arg, builder::styling, crate_description, crate_name, crate_version};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .error(styling::AnsiColor::Red.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

fn main() {
    let matches = Command::new(crate_name!())
        .styles(STYLES)
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            arg!(--port <VALUE> "Port to run the server on")
                .required(false)
                .default_value("6699"),
        )
        .arg(
            arg!(--host <VALUE> "Host to run the server on")
                .required(false)
                .default_value("127.0.0.1"),
        )
        .arg(
            arg!(--path <VALUE> "Repository to launch in VSCode")
                .required(false)
                .default_value("."),
        )
        .arg(
            arg!(--open "Open the server in the browser")
                .required(false)
                .default_value("false"),
        )
        .get_matches();

    let mut command = std::process::Command::new("nix");
    command
        .arg("run")
        // TODO: set in a way where we can change vscode server implementation easily
        .arg("github:nixos/nixpkgs/nixpkgs-unstable#openvscode-server")
        .arg("--")
        .arg(format!(
            "--port={}",
            matches
                .get_one::<String>("port")
                .expect("port has a default value")
        ))
        .arg(format!(
            "--host={}",
            matches
                .get_one::<String>("host")
                .expect("host has a default value")
        ))
        .args([
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

    let stdout_thread = thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            if !line.starts_with("Web UI available at") {
                continue;
            }
            let token = line.split("tkn=").collect::<Vec<&str>>()[1];
            let repo_vscode_url = format!(
                "http://{}:{}?tkn={}&folder={}",
                matches
                    .get_one::<String>("host")
                    .expect("host has a default value"),
                matches
                    .get_one::<String>("port")
                    .expect("port has a default value"),
                token,
                matches
                    .get_one::<String>("path")
                    .expect("path has a default value"),
            );

            println!("\n{}", repo_vscode_url);

            if matches
                .get_one::<String>("open")
                .expect("open has a default value")
                == "true"
            {
                let _ = std::process::Command::new("xdg-open")
                    .arg(repo_vscode_url)
                    .spawn()
                    .expect("Failed to open browser")
                    .wait();
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
