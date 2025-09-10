use crate::ShellState;
use crate::builtin::Builtin;
use crate::error::ShellError;
use crate::color::{paint, Fg};
use rustyline::history::History as RustylineHistory;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct History;

impl Builtin for History {
    fn name(&self) -> &'static str {
        "history"
    }

    fn run(&self, _argv: &[String], sh: &mut ShellState) -> Result<(), ShellError> {
        println!("{}", paint("Command History:", Fg::Cyan));
        println!();
        
        // Get history from rustyline
        let history = sh.rl.history();
        for (i, cmd) in history.iter().enumerate() {
            println!("  {}  {}", paint(&format!("{:3}", i + 1), Fg::Yellow), cmd);
        }
        
        if history.is_empty() {
            println!("  {}", paint("No commands in history", Fg::Red));
        }
        
        Ok(())
    }
}