mod builtin;
mod echo;
mod error;
mod reg;
mod repl;
mod rm;
mod touch;
mod cat;
mod cp;
mod mv;
mod color;
mod test_color;  // أضف هذا
mod ls;          // أضف هذا

pub use crate::builtin::Builtin;

use crate::repl::{ShellState, repl_loop};

fn main() {
    let sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
    };
    let reg = &mut reg::BuiltinRegistry::new();
    
    reg.register(echo::Echo);
    reg.register(rm::Rm);
    reg.register(touch::Touch);
    reg.register(cat::Cat);
    reg.register(cp::Cp);
    reg.register(mv::Mv);
    reg.register(test_color::TestColors);  // أضف هذا
    reg.register(ls::Ls);                   // أضف هذا

    repl_loop(sh, reg);
}