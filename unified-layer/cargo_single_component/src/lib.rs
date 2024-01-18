cargo_component_bindings::generate!();

use bindings::Guest;

struct Foo;

impl Guest for Foo {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
