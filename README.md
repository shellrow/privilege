[crates-badge]: https://img.shields.io/crates/v/privilege.svg
[crates-url]: https://crates.io/crates/privilege
[license-badge]: https://img.shields.io/crates/l/privilege.svg
[examples-url]: https://github.com/shellrow/privilege/tree/main/examples
# privilege [![Crates.io][crates-badge]][crates-url] ![License][license-badge]
Cross-platform library for administrative permission handling.

## Features
- Check Administrative Permissions: Easily determine if the current process has administrative or root permissions.
- Execute Commands with Administrative Privileges: Run commands with elevated permissions, ensuring secure and proper execution of critical operations.

## Supported platform
- Linux
- macOS
- Windows

## Usage
Add `privilege` to your dependencies  
```toml:Cargo.toml
[dependencies]
privilege = "0.3"
```

For more details, see [examples][examples-url] or doc.  
