
use std::collections::HashMap;
use crate::builtin::Builtin;


pub struct BuiltinRegistry {
    pub map: HashMap<&'static str, Box<dyn Builtin + Send + Sync>>,
}

impl BuiltinRegistry {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn register<B: Builtin + Send + Sync + 'static>(&mut self, b: B) {
        self.map.insert(b.name(), Box::new(b));
    }

    pub fn get(&self, name: &str) -> Option<&(dyn Builtin + Send + Sync)> {
        self.map.get(name).map(|b| b.as_ref())
    }
}