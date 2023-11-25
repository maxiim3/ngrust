![image.png](./asset/image.png)

# ngrust

An Angular-CLI inspired tool built with Rust as a learning-project purpose

## Crates.io

To use `ngrust` in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
ngrust = "1.0.1"
```

Then, you can use the `ngrust` command in your terminal to generate new components:

```bash
ngrust --gc <COMPONENT_NAME>
```

## GitHub

Clone the repository and install it locally:

```bash
git clone https://github.com/maxiim3/ngrust.git
cd ngrust
```

To test the CLI, run:

```bash
cargo run -- # Simulates ngrust
cargo run -- --gc # Simulates ngrust --gc
```

The `--` flag is used to escape the `cargo` command.

## Build and Install the Project

Build the project using the `--release` flag:

```bash
cargo build --release
```

This will create the executable binary inside `/target/release/`. Then, move the binary with **sudo** permission into your `PATH`. On macOS, it would be `/usr/local/bin`. Once you've done this, you should be able to use the `ngrust` command.

<style>
  *{
font-family:Inter, sans-serif;
}
h1, h2{ 
color:#93350d; 
}
p{font-size:18px;}
</style>
