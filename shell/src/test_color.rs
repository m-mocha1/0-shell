use crate::{Builtin, ShellState};
use crate::error::ShellError;
use crate::color::{paint, Fg, error_red};

pub struct TestColors;

impl Builtin for TestColors {
    fn name(&self) -> &'static str { "test_colors" }

    fn run(&self, _argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        println!("=== Color System Test ===");
        println!();
        
        // Test all colors
        println!("{}", paint("■ Red text", Fg::Red));
        println!("{}", paint("■ Green text", Fg::Green));
        println!("{}", paint("■ Blue text", Fg::Blue));
        println!("{}", paint("■ Yellow text", Fg::Yellow));
        println!("{}", paint("■ Magenta text", Fg::Magenta));
        println!("{}", paint("■ Cyan text", Fg::Cyan));
        println!("{}", paint("■ Default text", Fg::Default));
        
        println!();
        println!("{}", error_red("Error message in red"));
        
        println!();
        println!("=== File Type Colors Test ===");
        
        // Simulate different file types
        println!("📁 {}", paint("directory", Fg::Blue));
        println!("📄 normal_file.txt");
        println!("🔧 {}", paint("executable_file", Fg::Green));
        
        println!();
        println!("{}", paint("✅ If you see colors, everything works perfectly!", Fg::Green));
        println!("{}", paint("❌ If you see weird symbols, your terminal doesn't support colors", Fg::Red));
        
        Ok(())
    }
}