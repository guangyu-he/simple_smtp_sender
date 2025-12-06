#!/usr/bin/env -S rust-script
//! ```cargo
//! [dependencies]
//! tokio = {version = "1", features = ["process", "io-util", "macros", "rt-multi-thread"]}
//! ```

use std::env;
use std::sync::LazyLock;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

static MATURIN_USERNAME: LazyLock<String> =
    LazyLock::new(|| env::var("MATURIN_USERNAME").unwrap_or_default());
static MATURIN_PASSWORD: LazyLock<String> =
    LazyLock::new(|| env::var("MATURIN_PASSWORD").unwrap_or_default());

fn build_cmd(target: &str, no_sdist: bool) -> Command {
    let mut cmd = Command::new("maturin");
    cmd.arg("publish");

    cmd.args(["--features", "python"]);

    if !MATURIN_USERNAME.is_empty() {
        cmd.args(["-u", MATURIN_USERNAME.as_str()]);
    }

    if !MATURIN_PASSWORD.is_empty() {
        cmd.args(["-p", MATURIN_PASSWORD.as_str()]);
    }

    cmd.args(["--profile", "release"]);
    cmd.stdout(std::process::Stdio::piped());

    if !target.is_empty() {
        cmd.args(["--target", target]);
        cmd.arg("--zig");
    }
    if no_sdist {
        cmd.arg("--no-sdist");
    }
    cmd
}

async fn each_build(target: &str, no_sdist: bool) -> std::process::ExitCode {
    let mut command = build_cmd(target, no_sdist);
    let mut child = command.spawn().expect("Failed to run");
    let stdout = child.stdout.take().expect("failed to catch stdout");
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        println!("{}", line);
    }

    match child.wait().await {
        Ok(status) if status.success() => std::process::ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("Exit status: {}", status);
            std::process::ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::ExitCode::FAILURE
        }
    }
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    if MATURIN_PASSWORD.is_empty() {
        eprintln!("MATURIN_PASSWORD not set");
        return std::process::ExitCode::FAILURE;
    }

    let targets = vec![
        "", // default target: aarch64-apple-darwin
        "aarch64-unknown-linux-gnu",
        "aarch64-unknown-linux-musl",
        "x86_64-pc-windows-gnu",
        "x86_64-unknown-linux-gnu",
        "x86_64-unknown-linux-musl",
    ];

    for target in targets {
        println!(
            "Building for target: {}",
            if target.is_empty() { "default" } else { target }
        );
        let exit_code = each_build(target, if target.is_empty() { false } else { true }).await;
        if exit_code != std::process::ExitCode::SUCCESS {
            return exit_code;
        }
    }
    std::process::ExitCode::SUCCESS
}
