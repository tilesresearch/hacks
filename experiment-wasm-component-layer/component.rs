use wasm_component_layer::*;

const WASM: &[u8] = include_bytes!("component/add.wasm");

fn main() {
    let engine = Engine::new(wasmtime::Engine::default());
    let store = Store::new(&engine);
    //Load the WASM component into wasm_component_layer and invoke the add function as follows:

    let module = Module::new(&store, WASM).unwrap();
    let instance = Instance::new(&store, &module, "test", "add").unwrap();
    let add = instance.get_typed_func::<(i32, i32), i32>("add").unwrap();
    let result = add.call(2, 3).unwrap();
    assert_eq!(result, 5);
}

