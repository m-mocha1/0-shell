mod builtin;
mod echo;
mod error;
mod reg;
mod repl;

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};

fn main() {
    let sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
    };
    let reg = &mut reg::BuiltinRegistry::new();
    reg.register(echo::Echo);

    repl_loop(sh, reg);
}
