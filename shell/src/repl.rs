use crate::color::Fg;
use crate::color::paint;

use crate::reg::BuiltinRegistry;
use std::io;
use std::io::Write;
use std::path::PathBuf;

// Bonus imports
use crate::bonus_chaining::chain_commands;
use crate::bonus_help::print_help;
use crate::bonus_history::History;
use crate::bonus_pipes::parse_pipes;
#[derive(Debug)]
pub struct Command {
    pub argv: Vec<String>,
}
pub struct ShellState {
    pub cwd: PathBuf,
    pub running: bool,
    pub prompt_color: Fg,
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

pub fn repl_loop(sh: &mut ShellState, reg: &mut BuiltinRegistry) {
    let mut clear = false;
    let mut history = History::new();
    while sh.running {
        print!("{} $ :", sh.cwd.display());
        io::stdout().flush().ok();

        let mut input = String::new();
        let res = std::io::stdin().read_line(&mut input);
        let bytes_read = match res {
            Ok(n) => n,
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        };

        let input = input.trim_end().to_string();
        history.add(input.clone());

        if input.trim() == "help" {
            print_help();
            continue;
        }
        if input.trim() == "exit" {
            sh.running = false;
            break;
        }
        if input.trim() == "clear" {
            sh.running = false;
            clear = true;
            break;
        }
        if bytes_read == 0 {
            println!("exit");
            sh.running = false;
            break;
        }
        if input.trim().is_empty() {
            continue;
        }

        print!("{} $ :", sh.cwd.display());

        let commands = chain_commands(&input);
        for cmd_str in commands {
            let pipes = parse_pipes(cmd_str);
            for pipe_cmd in pipes {
                if let Some(cmd) = parts(pipe_cmd) {
                    let name = cmd.argv[0].as_str();
                    if let Some(builtin) = reg.get(name) {
                        builtin.run(&cmd.argv, sh);
                    } else {
                        println!("Command '{}' not found", name);
                    }
                }
            }
        }
    }
    sh.running = false;
    if clear {
        // Clear the terminal screen
        print!("\x1B[2J\x1B[1;1H"); // ansi escape code to clear screen and move cursor to top-left
        io::stdout().flush().ok();
        sh.running = true;
        repl_loop(sh, reg); // Restart the REPL loop
    }
}

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
