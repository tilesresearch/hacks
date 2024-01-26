# wasm-layer

> Experiment with cross-platform unified layer for Wasm and WebGPU.

## Usage

Run all the commands from the root unified-layer directory.

Install Rust:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install and set Rust nightly:

```
$ rustup toolchain install nightly && rustup override set nightly
```

Install wasm-tools:

```
$ cargo install wasm-tools
```

Install wasm-pack:

```
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Run the project:

```
$ cd single_component && \
rustup target add wasm32-unknown-unknown && \
cargo build && \
chmod +x build.sh && \
./build.sh && \
cd ../wasm-layer && \
cargo run
```

## For building component with cargo-component

```
$ cargo install cargo-component
```