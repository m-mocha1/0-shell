// Prompt with current directory feature stub
use std::env;

pub fn get_prompt() -> String {
    let cwd = env::current_dir().unwrap_or_default();
    format!("{} $ ", cwd.display())
}
