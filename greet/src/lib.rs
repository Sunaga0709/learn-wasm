#[allow(warnings)]
mod bindings;

use bindings::exports::mynamespace::greet::greetable::Guest;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "Wasm component".to_string()
    }

    fn greet(name: String) -> String {
        format!("Hello, {name}")
    }
}

bindings::export!(Component with_types_in bindings);
