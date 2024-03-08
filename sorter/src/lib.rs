#[allow(warnings)]
mod bindings;

use bindings::exports::macovedj::shapes::sorter::Guest;
use bindings::macovedj::shapes::goodmap::Goodmap;

struct Component;

impl Guest for Component {
    fn sort() -> Vec<String> {
        let map = Goodmap::new();
        let keys: Vec<String> = map.keys().into_iter().collect();
        keys
    }
}

bindings::export!(Component with_types_in bindings);
