use crate::error::ShellError;
use crate::{Builtin, ShellState};
use std::fs::{self, OpenOptions};
use std::path::Path;

pub struct Touch;

impl Builtin for Touch {
    fn name(&self) -> &'static str {
        "touch"
    }

    fn run(&self, argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        if argv.len() < 2 {
            eprintln!("touch: missing file operand");
            eprintln!("usage: touch <file>...");
            return Ok(());
        }

        for file in &argv[1..] {
            let path = Path::new(file);

            // Create the file if it does not exist, otherwise update modified time
            if !path.exists() {
                if let Err(e) = OpenOptions::new().create(true).write(true).open(path) {
                    eprintln!("touch: cannot create '{}': {}", file, e);
                }
            } else {
                // update timestamp (fallback: read+write to refresh mtime)
                if let Err(e) = fs::OpenOptions::new().append(true).open(path) {
                    eprintln!("touch: cannot update '{}': {}", file, e);
                }
            }
        }
        Ok(())
    }
}
