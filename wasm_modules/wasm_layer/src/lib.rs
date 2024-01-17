use wasm_component_layer::* ;

// The bytes of the component.
const WASM: &[u8] = include_bytes!("../../component.wasm");

pub fn run() {
    // Create a new engine for instantiating a component.
    let engine = Engine::new(wasmtime::Engine::default());

    // Create a store for managing WASM data and any custom user-defined state.
    let mut store = Store::new(&engine, ());

    // Parse the component bytes and load its imports and exports.
    let component = Component::new(&engine, WASM).unwrap();
    // Create a linker that will be used to resolve the component's imports, if any.
    let linker = Linker::default();
    // Create an instance of the component using the linker.
    let instance = linker.instantiate(&mut store, &component).unwrap();

    // Get the interface that the interface exports.
    let interface = instance
        .exports()
        .instance(&"test:guest/add".try_into().unwrap())
        .unwrap();
    // Get the function for selecting a list element.
    let addtwonumbers = interface
        .func("addtwonumbers")
        .unwrap()
        .typed::<(u32, u32), u32>()
        .unwrap();


    println!(
        "{}",
        addtwonumbers.call(&mut store, (2,5)).unwrap()
    );
    // Prints 'Calling select-nth(["a", "b", "c"], 1) == b'
}
