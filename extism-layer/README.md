# extism-layer

> Base setup for extism with Rust host SDK and guest PDK.

## TODO

- [ ] Wrap extism host and guest packages with tilekit with cargo subcommands

---

## Usage

### For host

Run the plugin

```shell
$ cargo run
# => {"count":3,"total":3,"vowels":"aeiouAEIOU"}
```

### For guest

Write a plugin

```shell
cargo build --target wasm32-unknown-unknown
```

Install extism CLI

```shell
curl https://get.extism.org/cli | sh
```

```shell
$ extism call target/wasm32-unknown-unknown/debug/my_plugin.wasm greet --input "Benjamin"
# => Hello, Benjamin!
```
