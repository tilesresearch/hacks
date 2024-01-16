# wasm-layer

> Experiment with cross-platform unified layer for Wasm and WebGPU.

## Usage

Install Rust:

`$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
`

Install wasm-tools:

`$ cargo install wasm-tools`

Install wasm-pack

`$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
`

Run the project:

`$ cd single_component && \
cargo build && \
chmod +x build.sh && \
cd ../wasm-layer && \
cargo run`
