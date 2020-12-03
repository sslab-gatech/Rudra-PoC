use std::process::{Command, ExitStatus};

use crate::poc::Metadata;

pub fn cargo_command(subcommand: &str, metadata: &Metadata) -> Command {
    let mut command = Command::new("cargo");

    if let Some(toolchain) = &metadata.test.cargo_toolchain {
        command.arg(format!("+{}", toolchain));
    }

    command.arg(subcommand);
    command.args(&metadata.test.cargo_flags);

    command
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
