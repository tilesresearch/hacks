use wasm_component_layer::*;

fn main() {
    // Create a new engine for instantiating a component.
    let engine = Engine::new(wasmtime::Engine::default());

    // Create a store for managing WASM data and any custom user-defined state.
    let mut store = Store::new(&engine, ());

    // Load and link the WASM component.
    wasm_component_layer::load_and_link_wasm(&engine, "./component/add_numbers.wasm", &mut store).unwrap();

    // Get the function to add two numbers.
    let add_function = wasm_component_layer::get_exported_function(&store, "add").unwrap();

    // Invoke the function with the arguments 2 and 3.
    let result = add_function.call(&[2i32, 3i32])[0];

    // Print the result.
    println!("The result is: {}", result);
}

//tiles add 3+5 ->5
//tiles : try using extism -> api referece, cli, how they work.
//
//


