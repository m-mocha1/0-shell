// Custom help command documenting built-in functionality

pub fn print_help() {
    println!("Shell Help:\n");
    println!("Built-in commands:");
    println!("- cd: Change directory");
    println!("- ls: List files");
    println!("- mkdir: Make directory");
    println!("- rm: Remove file/directory");
    println!("- cp: Copy file/directory");
    println!("- mv: Move file/directory");
    println!("- touch: Create empty file");
    println!("- echo: Print text");
    println!("Bonus features:");
    println!("- Command history, auto-completion, chaining, pipes, I/O redirection, environment variables, colorized output, SIGINT handling");
}
