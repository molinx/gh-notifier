# Github notifier

Simple app to show the number of github notifications in the system tray

## Requirements

- Rust

## Usage

1. Create a github token with the `notifications` scope
2. Create a `.cargo/config.toml` file from the example and set the `TOKEN`
3. Run `cargo tauri dev` to start a development run
4. Run `cargo tauri build` to get a Mac OS application
