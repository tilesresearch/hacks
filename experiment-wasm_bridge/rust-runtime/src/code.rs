use std::fmt::Write;

use wasm_bridge::*;

static GUEST_BYTES: &[u8] =
    include_bytes!("../../rust-guest/target/wasm32-unknown-unknown/release/rust_guest.wasm");

pub fn get_text() -> String {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), GUEST_BYTES).expect("should create module");

    let instance = Instance::new(&mut store, &module, &[]).expect("should create instance");

    let rate_number = instance
        .get_typed_func::<i32, i32>(&mut store, "add_three")
        .expect("should get add_three exported function");

    let mut text = String::new();

    for number in [15, 8, -20, 162, 1023] {
        let result = rate_number
            .call(&mut store, number)
            .expect("should call add_three");

        writeln!(text, "{number} + 3 = {result}").unwrap();
    }

    text
}