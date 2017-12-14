
all:
  @cargo build --target=wasm32-unknown-unknown --release
  @cargo install --force --git https://github.com/alexcrichton/wasm-gc
  @wasm-gc target/wasm32-unknown-unknown/release/wasm_rust_chip8.wasm web/chip8.wasm