# wasm-layer

> [!NOTE]  
> Hack concluded: crate not feasible as of now. Decided to use extism instead.

> Experiment with cross-platform unified layer for Wasm and WebGPU.

## Usage

Run all the commands from the root unified-layer directory.

Install Rust:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install and set Rust nightly:

```shell
rustup toolchain install nightly && rustup override set nightly
```

Install wasm-tools:

```shell
cargo install wasm-tools
```

Install wasm-pack:

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Run the project:

```shell
$ cd single_component && \
rustup target add wasm32-unknown-unknown && \
cargo build && \
chmod +x build.sh && \
./build.sh && \
cd ../wasm-layer && \
cargo run --target wasm32-unknown-unknown

# => Calling select-nth(["a", "b", "c"], 1) == b
```

## For building component with cargo-component

```shell
cargo install cargo-component
```