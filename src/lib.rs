/// Provides functionality for executing commands with administrative privileges.
///
/// This module contains structures and functions that facilitate the execution
/// of commands with elevated permissions. 
pub mod runas;
/// Provides functionality for user privileges.
///
/// This module contains functions that check user privileges. 
/// It enables checking if the current process has administrative permissions.
pub mod user;

#[cfg(test)]
mod tests {}
