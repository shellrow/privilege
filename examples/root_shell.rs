use privilege::runas::Command;

/// Get the name of the shell to be run.
///
/// On Windows, it defaults to "cmd". On Unix-like systems, it uses the value
/// of the SHELL environment variable, or "bash" if the variable is not set.
fn get_shell_name() -> String {
    #[cfg(windows)]
    {
        "cmd".to_string()
    }
    #[cfg(unix)]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "bash".into())
    }
}

fn main() {
    println!("Running a root shell ...");
    let mut cmd: Command = Command::new(get_shell_name());

    match cmd.run() {
        Ok(status) => println!("Shell exited with status: {}", status),
        Err(e) => println!("Failed to execute shell with root privileges. {}", e),
    }
}
