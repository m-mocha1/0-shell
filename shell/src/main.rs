mod repl;
mod exe;
mod reg;
mod builtin;
mod error;

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};

fn main() {
    let sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
    };

    repl_loop(sh)
}
