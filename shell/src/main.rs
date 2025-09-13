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
mod cd;
mod pwd;

// Bonus features
mod bonus_sigint;
mod bonus_autocomplete;
mod bonus_history;
mod bonus_prompt;
mod bonus_color;
mod bonus_chaining;
mod bonus_pipes;
mod bonus_redirection;
mod bonus_env_vars;
mod bonus_help;

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};

// Import bonus modules
use crate::bonus_sigint as sigint;
use crate::bonus_history as history;
use crate::bonus_autocomplete as autocomplete;
use crate::bonus_prompt as prompt;
use crate::bonus_chaining as chaining;
use crate::bonus_pipes as pipes;
use crate::bonus_redirection as redirection;
use crate::bonus_env_vars as env_vars;
use crate::bonus_help as help;

fn main() {
    // Setup SIGINT handler
    sigint::setup_sigint_handler();

    // Command history
    let mut cmd_history = history::History::new();

    // Shell prompt
    let shell_prompt = prompt::get_prompt();

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
