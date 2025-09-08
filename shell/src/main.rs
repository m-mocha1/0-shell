mod builtin;
mod cat;
mod color;
mod cp;
mod echo;
mod error;
mod ls;
mod mkdir;
mod mv;
mod reg;
mod repl;
mod rm;
mod touch;

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};

fn main() {
    let sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
        prompt_color: color::Fg::Default,
    };
    let reg = &mut reg::BuiltinRegistry::new();
    reg.register(cat::Cat);
    reg.register(color::Color);
    reg.register(cp::Cp);
    reg.register(echo::Echo);
    reg.register(ls::Ls);
    reg.register(mv::Mv);
    reg.register(rm::Rm);
    reg.register(touch::Touch);
    reg.register(mkdir::Mkdir);

    repl_loop(sh, reg);
}
