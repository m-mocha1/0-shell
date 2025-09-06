use crate::error::ShellError;
use std::fs;
use std::path::Path;

use crate::{Builtin, ShellState};

pub struct Rm;

impl Builtin for Rm {
    fn name(&self) -> &'static str {
        "rm"
    }

    fn run(&self, argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        if argv.len() < 2 {
            eprintln!("rm: missing operand");
            eprintln!("usage: rm [-r] <file_or_dir>...");
            return Ok(());
        }

        let mut recursive = false;
        let mut targets: Vec<&str> = Vec::new();

        // parse args
        for arg in &argv[1..] {
            if arg == "-r" {
                recursive = true;
            } else {
                targets.push(arg);
            }
        }

        if targets.is_empty() {
            eprintln!("rm: missing operand after '-r'");
            return Ok(());
        }

        // process each target
        for t in targets {
            let path = Path::new(t);

            if path.is_dir() {
                if recursive {
                    if let Err(e) = fs::remove_dir_all(path) {
                        eprintln!("rm: cannot remove '{}': {}", t, e);
                    }
                } else {
                    eprintln!("rm: cannot remove '{}': Is a directory", t);
                }
            } else if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}", t, e);
            }
        }
        Ok(())
    }
}
