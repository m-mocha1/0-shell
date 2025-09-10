use crate::ShellState;
use crate::builtin::Builtin;
use crate::error::ShellError;
use std::env;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cd;

impl Builtin for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn run(&self, argv: &[String], sh: &mut ShellState) -> Result<(), ShellError> {
        let target = if argv.len() <= 1 {
            env::var("HOME").unwrap_or_else(|_| String::from("/"))
        } else if argv[1].starts_with("~") {
            let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
            home + &argv[1][1..]
        } else {
            argv[1].clone()
        };

        let mut path = PathBuf::from(&target);
        if path.is_relative() {
            path = sh.cwd.join(path);
        }

        match env::set_current_dir(&path) {
            Ok(()) => {
                match path.canonicalize() {
                    Ok(abs) => sh.cwd = abs,
                    Err(_) => sh.cwd = path,
                }
            }
            Err(e) => eprintln!("cd: {}: {}", target, e),
        }

        Ok(())
    }
}
