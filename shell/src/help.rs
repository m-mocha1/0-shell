use crate::ShellState;
use crate::builtin::Builtin;
use crate::error::ShellError;
use crate::color::paint;
use crate::color::Fg;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Help;

impl Builtin for Help {
    fn name(&self) -> &'static str {
        "help"
    }

    fn run(&self, _argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        println!("{}", paint("0-shell Built-in Commands:", Fg::Cyan));
        println!();
        
        let commands = vec![
            ("cat", "Display file contents"),
            ("cd", "Change directory"),
            ("color", "Change prompt color (red, green, blue, cyan, yellow, magenta)"),
            ("cp", "Copy files or directories"),
            ("echo", "Print text to stdout"),
            ("help", "Show this help message"),
            ("history", "Show command history"),
            ("ls", "List directory contents"),
            ("mkdir", "Create directories"),
            ("mv", "Move/rename files or directories"),
            ("rm", "Remove files or directories"),
            ("touch", "Create empty files or update timestamps"),
            ("clear", "Clear the terminal screen"),
            ("exit", "Exit the shell"),
        ];

        for (cmd, desc) in commands {
            println!("  {}  {}", paint(cmd, Fg::Green), desc);
        }
        
        println!();
        println!("{}", paint("Bonus Features:", Fg::Yellow));
        println!("  • Colored prompt with current directory");
        println!("  • Basic file operations");
        println!("  • Quote and escape character support");
        println!("  • Command history with arrow keys");
        println!("  • Ctrl+C to exit shell");
        
        Ok(())
    }
}