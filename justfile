wasm:
    cargo build --target wasm32-unknown-unknown --manifest-path replay_tools/Cargo.toml
    wasm-bindgen --target bundler --out-dir ./src/wasm target/wasm32-unknown-unknown/debug/replay_tools.wasm

wasm-release:
    cargo build --release --target wasm32-unknown-unknown --manifest-path replay_tools/Cargo.toml
    wasm-bindgen --target bundler --out-dir ./src/wasm target/wasm32-unknown-unknown/release/replay_tools.wasm

fmt:
    cargo +nightly fmt