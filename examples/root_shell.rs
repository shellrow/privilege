use privilege::runas::Command;

fn get_shell_name() -> String {
    #[cfg(windows)]
    {"cmd".to_string()}
    #[cfg(unix)]
    {std::env::var("SHELL").unwrap_or_else(|_| "bash".into())}
}

fn main() {
    println!("Run a root shell:");
    match Command::new(get_shell_name()).run() {
        Ok(status) => println!("Exit Status: {}", status),
        Err(e) => println!("Failed to execute. {}",e),
    }
}