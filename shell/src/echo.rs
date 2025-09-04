use crate::ShellState;
use crate::builtin::Builtin;
use crate::error::ShellError;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Echo;

impl Builtin for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }
    fn run(&self, argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        if argv.len() <= 1 {
            println!();
        } else {
            println!("{}", argv[1..].join(" "));
        }
        Ok(())
    }
}
