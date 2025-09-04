use std::collections::HashMap;
use crate::{Builtin, ShellState};
use crate::repl::Command; // Add this line if Command is defined in command.rs
use crate::error::ShellError;
pub struct BuiltinRegistry {
    map: HashMap<&'static str, Box<dyn Builtin + Send + Sync>>,
}

impl BuiltinRegistry {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn register<B: Builtin + Send + Sync + 'static>(&mut self, b: B) {
        self.map.insert(b.name(), Box::new(b));
    }
    pub fn get(&self, name: &str) -> Option<&(dyn Builtin + Send + Sync)> {
        self.map.get(name).map(|b| b.as_ref())
    }
}
pub fn execute(cmd: Command, sh: &mut ShellState, reg: &BuiltinRegistry) {
    if cmd.argv.is_empty() {
        return;
    }

    let name = &cmd.argv[0];
    if let Some(b) = reg.get(name) {
        match b.run(&cmd.argv, sh) {
            Ok(()) => {}
            Err(ShellError::Usage(msg)) => eprintln!("{}: {}", name, msg),
            Err(ShellError::NotFound(cmd)) => eprintln!("Command '{}' not found", cmd),
            Err(ShellError::Io(e)) => eprintln!("{}: {}", name, e),
        }
    } else {
        println!("Command '{}' not found", name);
    }
}
