- Install Rust, follow the [official instructions](https://www.rust-lang.org/tools/install).
- Install WebAssembly target
    ```rust
    rustup target add wasm32-unknown-unknown
    ```
- Install Trunk for managing deployment and packaging
    ```rust
    # See https://trunkrs.dev/#install for further details
    cargo install --locked trunk
    ```
- Create rust project
