use std::collections::HashMap;
use std::fs;
#[allow(warnings)]
mod bindings;

use crate::bindings::exports::macovedj::resource_map::resource_map::{Guest, GuestResourceMap};

struct Component;

impl GuestResourceMap for Component {
    fn new() -> Self {
        Self
    }
    fn insert(&self, name: String, job: String) {
        let content = fs::read("./foo");
        let bytes = &content.expect("should exist");
        let mut data: HashMap<String, String> =
            serde_json::from_str(std::str::from_utf8(bytes).unwrap()).unwrap();
        data.insert(name.clone(), job);
        fs::write("./foo", &serde_json::to_string(&data).unwrap());
    }
    fn get(&self, name: String) -> Option<String> {
        let content = fs::read("./foo");
        let bytes = &content.expect("should exist");
        let mut data: HashMap<String, String> =
            serde_json::from_str(std::str::from_utf8(bytes).unwrap()).unwrap();
        data.get(&name).cloned()
    }
    fn keys(&self) -> Vec<String> {
        let content = fs::read("./foo");
        let bytes = &content.expect("should exist");
        let mut data: HashMap<String, String> =
            serde_json::from_str(std::str::from_utf8(bytes).unwrap()).unwrap();
        data.keys().map(|k| k.clone()).collect()
    }
}

impl Guest for Component {
    type ResourceMap = Self;
}

bindings::export!(Component with_types_in bindings);
