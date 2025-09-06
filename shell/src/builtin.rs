use crate::ShellState;
use crate::error::ShellError;
pub trait Builtin {
    fn name(&self) -> &'static str;
    fn run(&self, argv: &[String], sh: &mut ShellState) -> Result<(), ShellError>;
}
