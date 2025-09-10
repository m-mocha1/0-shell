mod builtin;
mod cat;
mod color;
mod cp;
mod echo;
mod error;
mod help;
mod history;
mod ls;
mod mkdir;
mod mv;
mod reg;
mod repl;
mod rm;
mod touch;
mod cd;

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use rustyline::DefaultEditor;

fn main() {
    // Initialize rustyline editor
    let rl = DefaultEditor::new().expect("Failed to initialize rustyline");
    
    let sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
        prompt_color: color::Fg::Default,
        interrupted: Arc::new(AtomicBool::new(false)),
        rl,
    };
    let reg = &mut reg::BuiltinRegistry::new();
    reg.register(cat::Cat);
    reg.register(color::Fg::Default);
    reg.register(cp::Cp);
    reg.register(cd::Cd);
    reg.register(echo::Echo);
    reg.register(help::Help);
    reg.register(history::History);
    reg.register(ls::Ls);
    reg.register(mv::Mv);
    reg.register(rm::Rm);
    reg.register(touch::Touch);
    reg.register(mkdir::Mkdir);

    repl_loop(sh, reg);
}
