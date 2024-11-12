#[allow(warnings)]
mod bindings;

use bindings::exports::mynamespace::greet::sayable::Guest;
use bindings::mynamespace::greet::greetable::{greet, name};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn say() -> String {
        let name = name();
        let greetings = greet(&name);

        let mut buffer = Vec::new();
        ferris_says::say(&greetings, 80, &mut buffer).unwrap();

        String::from_utf8(buffer).unwrap()
    }
}

bindings::export!(Component with_types_in bindings);