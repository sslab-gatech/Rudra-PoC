use std::{path::Path, process::ExitStatus};

use crate::poc::TestMetadata;

use duct::{cmd, Expression};

pub fn cargo_command(
    subcommand: &str,
    metadata: &TestMetadata,
    path: impl AsRef<Path>,
) -> Expression {
    let command_vec = cargo_command_vec(subcommand, metadata);
    cmd(&command_vec[0], &command_vec[1..]).dir(path.as_ref())
}

pub fn cargo_command_str(subcommand: &str, metadata: &TestMetadata) -> String {
    let command_vec = cargo_command_vec(subcommand, metadata);
    command_vec.join(" ")
}

pub fn cargo_command_vec(subcommand: &str, metadata: &TestMetadata) -> Vec<String> {
    let mut command_vec = vec![String::from("cargo")];

    if let Some(toolchain) = &metadata.cargo_toolchain {
        command_vec.push(format!("+{}", toolchain));
    }

    command_vec.push(String::from(subcommand));

    for flag in &metadata.cargo_flags {
        command_vec.push(flag.clone());
    }

    command_vec
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
