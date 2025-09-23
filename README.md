# How to install

1. Install rustup and toolchains.

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup install 1.86.0
    rustup default 1.86.0
    ```

2. Add target for WASM.

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install trunk.

    ```bash
    cargo install -f --locked trunk --version 0.21.13
    ```

# How to run

1. Run it with debug feature.

   ```bash
   trunk serve --features "debug"
   ```

# How to release

```bash
trunk build --release --public-url /rhb
aws s3 cp --recursive dist/ s3://${MY_BUCKET}/rhb/
```
