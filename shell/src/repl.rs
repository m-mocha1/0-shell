use crate::reg::BuiltinRegistry;
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
    while sh.running {
        // REPL logic goes here
        print!("{} $ :", sh.cwd.display());
        //this to block the code until user input
        io::stdout().flush().ok();

        //this holds the user input
        let mut input = String::new();
        //this reads the user input and returns the number of bytes read
        //if 0 it means the user pressed ctrl+D
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap_or(0);

        // if user types exit
        if input.trim() == "exit" {
            sh.running = false;
            break;
        }
        // is user pressed Enter without any input
        if input.trim().is_empty() {
            continue;
        }
        // this for ctrl+D
        if bytes_read == 0 {
            println!("exit");
            sh.running = false;
            break;
        }

        let Some(cmd) = parts(&input) else {
            continue;
        };
        let name = cmd.argv[0].as_str();
        if let Some(builtin) = reg.get(name) {
            builtin.run(&cmd.argv, sh);
        } else {
            println!("Command '{}' not found", name);
        }
    }
    sh.running = false;
}

/*
A single backslash \ on its own is special (escape character).
To represent it as a char, you must escape it:

if the current character is a backslash \, and we’re not inside single quotes,
 then treat it as an escape.

 '\''
This is also a character literal.
The outer ' starts a char literal.
Inside, we want the character ' (a single quote).
To write it, we escape it with \.

the character ' (single quote)

'\\' → match the backslash character
'\'' → match the single quote character
'"' → match the double quote character
*/
pub fn parts(line: &str) -> Option<Command> {
    let mut args = Vec::<String>::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut in_double = false;

    let mut it = line.chars().peekable();
    while let Some(ch) = it.next() {
        // to detect spaces outside quotes and split accordingly
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
