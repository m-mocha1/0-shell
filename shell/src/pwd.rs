use crate::ShellState;
use crate::builtin::Builtin;
use crate::error::ShellError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pwd;

impl Builtin for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn run(&self, _argv: &[String], sh: &mut ShellState) -> Result<(), ShellError> {
        println!("{}", sh.cwd.display());
        Ok(())
    }
}
