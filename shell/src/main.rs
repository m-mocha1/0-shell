mod builtin;
mod cat;
mod cd;
mod color;
mod cp;
mod echo;
mod error;
mod ls;
mod mkdir;
mod mv;
mod pwd;
mod reg;
mod repl;
mod rm;
mod touch;

// Bonus features
mod bonus_chaining;
mod bonus_help;
mod bonus_history;
mod bonus_pipes;

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};

// Import bonus modules
use crate::bonus_chaining as chaining;
use crate::bonus_help as help;
use crate::bonus_history as history;
use crate::bonus_pipes as pipes;

fn main() {
    // Command history
    let mut cmd_history = history::History::new();

    // Shell prompt

    let sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
        prompt_color: color::Fg::Default,
    };
    let reg = &mut reg::BuiltinRegistry::new();
    reg.register(cat::Cat);
    reg.register(color::Fg::Default);
    reg.register(cp::Cp);
    reg.register(cd::Cd);
    reg.register(echo::Echo);
    reg.register(ls::Ls);
    reg.register(mv::Mv);
    reg.register(rm::Rm);
    reg.register(touch::Touch);
    reg.register(mkdir::Mkdir);
    reg.register(cd::Cd);
    reg.register(pwd::Pwd);

    // Print help on startup
    help::print_help();

    repl_loop(sh, reg);
}
