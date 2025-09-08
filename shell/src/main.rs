mod builtin;
mod echo;
mod error;
mod reg;
mod repl;
mod rm;
mod touch;
 mod ls;
mod mkdir;

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};

fn main() {
    let   sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
    };
    let  reg = &mut reg::BuiltinRegistry::new();
    reg.register(echo::Echo);
    reg.register(rm::Rm);
    reg.register(touch::Touch);
    reg.register(ls::Ls);
    reg.register(mkdir::Mkdir);
    repl_loop(  sh,  reg);
}
