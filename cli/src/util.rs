use std::{path::Path, process::ExitStatus, str::FromStr};

use crate::poc::TestMetadata;

use chrono::{DateTime, Local};
use duct::{cmd, Expression};

pub fn cargo_command(
    subcommand: &str,
    metadata: &TestMetadata,
    path: impl AsRef<Path>,
) -> Expression {
    let command_vec = cargo_command_vec(subcommand, metadata);
    cmd_remove_cargo_envs(cmd(&command_vec[0], &command_vec[1..]).dir(path.as_ref()))
}

pub fn cargo_command_vec(subcommand: &str, metadata: &TestMetadata) -> Vec<String> {
    let mut command_vec = vec![String::from("cargo")];

    command_vec.push(String::from(subcommand));

    for flag in &metadata.cargo_flags {
        command_vec.push(flag.clone());
    }

    command_vec
}

pub fn cmd_remove_cargo_envs(mut expression: Expression) -> Expression {
    for env_name in &[
        "CARGO",
        "CARGO_HOME",
        "CARGO_MANIFEST_DIR",
        "CARGO_PKG_AUTHORS",
        "CARGO_PKG_DESCRIPTIOn",
        "CARGO_PKG_HOMEPAGE",
        "CARGO_PKG_LICENSE",
        "CARGO_PKG_LICENSE_FILE",
        "CARGO_PKG_NAME",
        "CARGO_PKG_REPOSITORY",
        "CARGO_PKG_VERSION",
        "CARGO_PKG_VERSION_MAJOR",
        "CARGO_PKG_VERSION_MINOR",
        "CARGO_PKG_VERSION_PATCH",
        "CARGO_PKG_VERSION_PRE",
        "RUSTUP_HOME",
        "RUSTUP_TOOLCHAIN",
        "RUSTUP_RECURSION",
    ] {
        expression = expression.env_remove(env_name);
    }
    expression
}

pub fn cmd_run_silent(expression: Expression, path: impl AsRef<Path>) -> Expression {
    expression.stdout_null().stderr_null().dir(path.as_ref())
}

// https://man7.org/linux/man-pages/man7/signal.7.html
pub fn signal_name(signal: i32) -> &'static str {
    match signal {
        1 => "SIGHUP",
        2 => "SIGINT",
        3 => "SIGQUIT",
        4 => "SIGILL",
        5 => "SIGTRAP",
        6 => "SIGABRT",
        7 => "SIGBUS",
        8 => "SIGFPE",
        9 => "SIGKILL",
        10 => "SIGUSR1",
        11 => "SIGSEGV",
        12 => "SIGUSR2",
        13 => "SIGPIPE",
        14 => "SIGALRM",
        15 => "SIGTERM",
        16 => "SIGTKFLT",
        17 => "SIGCHLD",
        18 => "SIGCONT",
        19 => "SIGSTOP",
        20 => "SIGTSTP",
        21 => "SIGTTIN",
        22 => "SIGTTOU",
        23 => "SIGURG",
        24 => "SIGXCPU",
        25 => "SIGXFSZ",
        26 => "SIGVTARLM",
        27 => "SIGPROF",
        28 => "SIGWINCH",
        29 => "SIGIO",
        30 => "SIGPWR",
        31 => "SIGSYS",
        _ => "Unknown",
    }
}

pub fn exit_status_string(exit_status: &ExitStatus) -> String {
    use std::os::unix::process::ExitStatusExt;

    if let Some(signal) = exit_status.signal() {
        format!(
            "Terminated with signal {} ({})",
            signal,
            signal_name(signal)
        )
    } else if let Some(return_code) = exit_status.code() {
        format!("Return code {}", return_code)
    } else {
        String::from("Unknown return status")
    }
}

/// Local date in "%Y-%m-%d" format
pub fn today_str() -> String {
    let local_now: DateTime<Local> = Local::now();
    local_now.format("%Y-%m-%d").to_string()
}

pub fn today_toml_date() -> toml::value::Datetime {
    FromStr::from_str(&today_str()).unwrap()
}
