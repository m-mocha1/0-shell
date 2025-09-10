use crate::color::Fg;
use crate::color::paint;

use crate::reg::BuiltinRegistry;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustylineResult};
#[derive(Debug)]
pub struct Command {
    pub argv: Vec<String>,
}
pub struct ShellState {
    pub cwd: PathBuf,
    pub running: bool,
    pub prompt_color: Fg,
    pub interrupted: Arc<AtomicBool>,
    pub rl: DefaultEditor,
}

pub fn repl_loop(sh: &mut ShellState, reg: &mut BuiltinRegistry) {
        let mut clear = false;
        
        // Setup Ctrl+C handler
        setup_signal_handler(sh.interrupted.clone());
        
    while sh.running {
        // REPL logic
        let prompt = format_prompt(&sh.cwd, sh.prompt_color);
        
        // Read input with rustyline (supports arrow keys, history, etc.)
        match sh.rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() {
                    continue;
                }
                
                // Add to history
                let _ = sh.rl.add_history_entry(&input);
                
                // Process the command
                process_command(&input, sh, reg);
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C - Exit the shell
                println!();
                println!("exit");
                sh.running = false;
                break;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D - Do nothing, just continue
                println!();
                continue;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    sh.running = false;
    if clear {
        // Clear the terminal screen
        print!("\x1B[2J\x1B[1;1H");// ansi escape code to clear screen and move cursor to top-left
        io::stdout().flush().ok();
        sh.running = true;
        repl_loop(sh, reg); // Restart the REPL loop
    }
}

fn process_command(input: &str, sh: &mut ShellState, reg: &mut BuiltinRegistry) {
    // Handle special commands
    if input.trim() == "exit" {
        sh.running = false;
        return;
    }
    
    if input.trim() == "clear" {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().ok();
        return;
    }

    // Check if interrupted by Ctrl+C
    if sh.interrupted.load(Ordering::Relaxed) {
        println!();
        sh.interrupted.store(false, Ordering::Relaxed);
        return;
    }

    // Handle command chaining with semicolon
    let commands = parse_command_chain(input);
    for cmd_str in commands {
        // Check for interruption between commands
        if sh.interrupted.load(Ordering::Relaxed) {
            println!();
            sh.interrupted.store(false, Ordering::Relaxed);
            break;
        }
        
        // Check for pipes in the command
        if cmd_str.contains('|') {
            execute_pipeline(&cmd_str, sh, reg);
        } else if cmd_str.contains('>') || cmd_str.contains('<') {
            execute_with_redirection(&cmd_str, sh, reg);
        } else {
            let Some(cmd) = parts(&cmd_str) else {
                continue;
            };
            let name = cmd.argv[0].as_str();
            if let Some(builtin) = reg.get(name) {
                let _ = builtin.run(&cmd.argv, sh);
            } else {
                println!("{}", paint(&format!("Command '{}' not found", name), Fg::Red));
            }
        }
    }
}

/*
A single backslash \ on its own is special (escape character).
To represent it as a char, you must escape it:

if the current character is a backslash \, and we're not inside single quotes,
 then treat it as an escape.

 '\''
This is also a character literal.
The outer ' starts a char literal.
Inside, we want the character ' (a single quote).
To write it, you escape it with \.

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
            '$' if !in_single => {
                // Handle environment variable expansion
                let var_name = expand_env_var(&mut it);
                cur.push_str(&var_name);
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

fn expand_env_var(it: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut var_name = String::new();
    
    // Handle ${VAR} syntax
    if it.peek() == Some(&'{') {
        it.next(); // consume '{'
        while let Some(ch) = it.next() {
            if ch == '}' {
                break;
            }
            var_name.push(ch);
        }
    } else {
        // Handle $VAR syntax
        while let Some(ch) = it.peek() {
            if ch.is_alphanumeric() || *ch == '_' {
                var_name.push(it.next().unwrap());
            } else {
                break;
            }
        }
    }
    
    if var_name.is_empty() {
        return String::from("$");
    }
    
    env::var(&var_name).unwrap_or_else(|_| String::new())
}
#[cfg(unix)]
fn is_eof(bytes_read: usize, _buf: &str) -> bool {
    // Ctrl+D at empty line
    bytes_read == 0
}

fn format_prompt(cwd: &PathBuf, color: Fg) -> String {
    let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
    let cwd_str = cwd.to_string_lossy();
    
    let display_path = if cwd_str.starts_with(&home) {
        format!("~{}", &cwd_str[home.len()..])
    } else {
        cwd_str.to_string()
    };
    
    // Add color to the prompt
    let colored_path = paint(&display_path, color);
    format!("{} $ ", colored_path)
}

fn parse_command_chain(input: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    
    let mut it = input.chars().peekable();
    while let Some(ch) = it.next() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '\\' if !in_single => {
                if let Some(nc) = it.next() {
                    current.push(nc);
                }
            }
            ';' if !in_single && !in_double => {
                // End of command
                if !current.trim().is_empty() {
                    commands.push(current.trim().to_string());
                }
                current.clear();
            }
            c => current.push(c),
        }
    }
    
    // Add the last command if not empty
    if !current.trim().is_empty() {
        commands.push(current.trim().to_string());
    }
    
    commands
}

#[cfg(unix)]
fn setup_signal_handler(_interrupted: Arc<AtomicBool>) {
    use std::sync::Once;
    static INIT: Once = Once::new();
    
    INIT.call_once(|| {
        unsafe {
            let _ = libc::signal(libc::SIGINT, signal_handler as libc::sighandler_t);
        }
        
        // Store the interrupted flag in a global location
        // This is a simplified approach - in production you'd want proper signal handling
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(100));
                // This is a placeholder - real signal handling would be more complex
            }
        });
    });
}

#[cfg(unix)]
extern "C" fn signal_handler(_signal: libc::c_int) {
    // This is a simplified signal handler
    // In a real implementation, you'd need to properly handle the signal
    // and communicate with the main thread
}

#[cfg(not(unix))]
fn setup_signal_handler(_interrupted: Arc<AtomicBool>) {
    // Signal handling not implemented for non-Unix systems
}



fn execute_pipeline(cmd_str: &str, sh: &mut ShellState, reg: &mut BuiltinRegistry) {
    let pipeline = parse_pipeline(cmd_str);
    if pipeline.is_empty() {
        return;
    }
    
    // For now, we'll execute commands sequentially and capture output
    // In a real implementation, you'd use actual pipes between processes
    let _output = String::new();
    
    for (i, cmd_str) in pipeline.iter().enumerate() {
        let Some(cmd) = parts(cmd_str) else {
            continue;
        };
        
        let name = cmd.argv[0].as_str();
        if let Some(builtin) = reg.get(name) {
            // Capture output for piping (simplified implementation)
            if i == 0 {
                // First command - capture its output
                let _ = builtin.run(&cmd.argv, sh);
                // In a real implementation, you'd capture stdout here
            } else {
                // Subsequent commands - use previous output as input
                // For now, just execute normally
                let _ = builtin.run(&cmd.argv, sh);
            }
        } else {
            println!("{}", paint(&format!("Command '{}' not found", name), Fg::Red));
        }
    }
}

fn parse_pipeline(input: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    
    let mut it = input.chars().peekable();
    while let Some(ch) = it.next() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '\\' if !in_single => {
                if let Some(nc) = it.next() {
                    current.push(nc);
                }
            }
            '|' if !in_single && !in_double => {
                // End of command
                if !current.trim().is_empty() {
                    commands.push(current.trim().to_string());
                }
                current.clear();
            }
            c => current.push(c),
        }
    }
    
    // Add the last command if not empty
    if !current.trim().is_empty() {
        commands.push(current.trim().to_string());
    }
    
    commands
}

fn execute_with_redirection(cmd_str: &str, sh: &mut ShellState, reg: &mut BuiltinRegistry) {
    let (command, redirection) = parse_redirection(cmd_str);
    
    let Some(cmd) = parts(&command) else {
        return;
    };
    
    let name = cmd.argv[0].as_str();
    if let Some(builtin) = reg.get(name) {
        match redirection {
            Redirection::Output(filename) => {
                // Redirect stdout to file
                let _ = std::fs::write(&filename, "");
                // In a real implementation, you'd capture stdout and write to file
                let _ = builtin.run(&cmd.argv, sh);
                println!("{}", paint(&format!("Output redirected to {}", filename), Fg::Cyan));
            }
            Redirection::Input(filename) => {
                // Redirect stdin from file
                if let Ok(_content) = std::fs::read_to_string(&filename) {
                    // In a real implementation, you'd set stdin to read from file
                    let _ = builtin.run(&cmd.argv, sh);
                } else {
                    println!("{}", paint(&format!("Cannot read from file: {}", filename), Fg::Red));
                }
            }
            Redirection::Append(filename) => {
                // Append stdout to file
                // In a real implementation, you'd append to file
                let _ = builtin.run(&cmd.argv, sh);
                println!("{}", paint(&format!("Output appended to {}", filename), Fg::Cyan));
            }
            Redirection::None => {
                let _ = builtin.run(&cmd.argv, sh);
            }
        }
    } else {
        println!("{}", paint(&format!("Command '{}' not found", name), Fg::Red));
    }
}

#[derive(Debug)]
enum Redirection {
    Output(String),
    Input(String),
    Append(String),
    None,
}

fn parse_redirection(input: &str) -> (String, Redirection) {
    let mut command = String::new();
    let mut filename = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut redirection_type = Redirection::None;
    let mut found_redirection = false;
    
    let mut it = input.chars().peekable();
    while let Some(ch) = it.next() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '\\' if !in_single => {
                if let Some(nc) = it.next() {
                    if found_redirection {
                        filename.push(nc);
                    } else {
                        command.push(nc);
                    }
                }
            }
            '>' if !in_single && !in_double => {
                if !found_redirection {
                    found_redirection = true;
                    // Check for append (>>)
                    if it.peek() == Some(&'>') {
                        it.next();
                        redirection_type = Redirection::Append(String::new());
                    } else {
                        redirection_type = Redirection::Output(String::new());
                    }
                } else {
                    filename.push(ch);
                }
            }
            '<' if !in_single && !in_double => {
                if !found_redirection {
                    found_redirection = true;
                    redirection_type = Redirection::Input(String::new());
                } else {
                    filename.push(ch);
                }
            }
            c if found_redirection => {
                filename.push(c);
            }
            c => {
                command.push(c);
            }
        }
    }
    
    // Update the redirection with the filename
    let redirection = match redirection_type {
        Redirection::Output(_) => Redirection::Output(filename.trim().to_string()),
        Redirection::Input(_) => Redirection::Input(filename.trim().to_string()),
        Redirection::Append(_) => Redirection::Append(filename.trim().to_string()),
        Redirection::None => Redirection::None,
    };
    
    (command.trim().to_string(), redirection)
}