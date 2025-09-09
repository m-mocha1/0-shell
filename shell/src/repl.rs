use crate::reg::BuiltinRegistry;
use crate::error::ShellError;
use crate::color::{paint, error_red, Fg};
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Command {
    pub argv: Vec<String>,
}

pub struct ShellState {
    pub cwd: PathBuf,
    pub running: bool,
}

pub fn repl_loop(sh: &mut ShellState, reg: &mut BuiltinRegistry) {
    println!("{}", paint("Welcome to 0-shell with noor's colors! 🎨", Fg::Cyan));
    println!("{}", paint("Type 'test_colors' to test colors", Fg::Yellow));
    println!();

    while sh.running {
        print!("{} {} ", 
            paint(&format!("{}", sh.cwd.display()), Fg::Blue),
            paint("$", Fg::Green)
        );
        io::stdout().flush().ok();

        let mut input = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap_or(0);

        if input.trim() == "exit" {
            println!("{}", paint("Goodbye! 👋", Fg::Cyan));
            sh.running = false;
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        if bytes_read == 0 {
            println!("{}", paint("exit", Fg::Yellow));
            sh.running = false;
            break;
        }

        let Some(cmd) = parts(&input) else {
            continue;
        };
        
        let name = cmd.argv[0].as_str();
        if let Some(builtin) = reg.get(name) {
            match builtin.run(&cmd.argv, sh) {
                Ok(()) => {}
                Err(ShellError::Usage(msg)) => {
                    eprintln!("{}: {}", error_red(name), msg);
                }
                Err(ShellError::NotFound(file)) => {
                    eprintln!("{}: {}: No such file or directory", 
                        error_red(name), 
                        paint(&file, Fg::Yellow)
                    );
                }
                Err(ShellError::Io(e)) => {
                    eprintln!("{}: {}", error_red(name), e);
                }
            }
        } else {
            println!("Command '{}' not found", 
                paint(name, Fg::Red)
            );
        }
    }
    sh.running = false;
}

pub fn parts(line: &str) -> Option<Command> {
    let mut args = Vec::<String>::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut in_double = false;

    let mut it = line.chars().peekable();
    while let Some(ch) = it.next() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '\\' if !in_single => {
                if let Some(nc) = it.next() {
                    cur.push(nc);
                }
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !cur.is_empty() {
                    args.push(std::mem::take(&mut cur));
                }
                while let Some(peek) = it.peek() {
                    if peek.is_whitespace() {
                        it.next();
                    } else {
                        break;
                    }
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        args.push(cur);
    }
    if args.is_empty() {
        None
    } else {
        Some(Command { argv: (args) })
    }
}