mod repl;

use crate::repl::{ShellState, repl_loop};

fn main() {
    let sh = &mut repl::ShellState {
        cwd: std::env::current_dir().unwrap(),
        running: true,
    };
    // let register = vec!["cd", "ls", "exit"];

    repl_loop(sh)
}
