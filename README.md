# NgRust

An angular-cli ispired tool build with Rust as a learning-project purpose

## Installation

Use github clone and install the repo locally.
To test the CLI run  

```bash
cargo run -- # simulates ngrust
```

```bash
cargo run -- --gc # simulates ngrust --gc
```
> the `--`flag is used to escape the `cargo` command.

## Build and install the project

Build the project using the `--release` flag
```bash
cargo build --release
```

- This will create the executable binary inside `/target/release/`
- Then move the Binary with **sudo** permission into you `PATH`
  - On Mac it would be `/usr/local/bin`
- Once you've done this, you should be able to use `ngrust` command

