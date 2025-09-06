use std::fs;
use std::path::{Path, PathBuf};

use crate::builtin::Builtin;
use crate::error::ShellError;
use crate::repl::ShellState;

pub struct Mkdir;

impl Builtin for Mkdir {
    fn name(&self) -> &'static str { "mkdir" }

    fn run(&self, argv: &[String], sh: &mut ShellState) -> Result<(), ShellError> {
        let mut targets: Vec<PathBuf> = Vec::new();
        let mut parents = false; // يعبر عن -p

        // argv[0] = "mkdir"
        for a in argv.iter().skip(1) {
            if a == "-p" {
                parents = true;
                continue;
            }
            let p = PathBuf::from(a);
            targets.push(if p.is_absolute() { p } else { sh.cwd.join(p) });
        }

        if targets.is_empty() {
            eprintln!("mkdir: missing operand");
            return Ok(());
        }

        for p in targets {
            let res = if parents {
                fs::create_dir_all(&p)
            } else {
                fs::create_dir(&p)
            };

            if let Err(e) = res {
                eprintln!("mkdir: {}: {}", p.display(), e);
            }
        }

        Ok(())
    }
}
