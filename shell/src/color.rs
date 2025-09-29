use crate::ShellState;
use crate::Builtin;
use crate::error::ShellError;
use std::io;

use std::io::Write;


// src/color.rs

pub struct Color;
#[derive(Debug, PartialEq, Clone, Copy)]

pub enum Fg {
    Red,
    Green,
    Blue,
    Cyan,
    Yellow,
    Magenta,
    Default,
}

fn code(c: Fg) -> &'static str {
    match c {
        Fg::Red => "\x1b[31m",
        Fg::Green => "\x1b[32m",
        Fg::Yellow => "\x1b[33m",
        Fg::Blue => "\x1b[34m",
        Fg::Magenta => "\x1b[35m",
        Fg::Cyan => "\x1b[36m",
        Fg::Default => "\x1b[39m",
    }
}


impl Builtin for Fg {
    fn name(&self) -> &'static str {
        "color"
    }
    fn run(&self, args: &[String], _state: &mut crate::repl::ShellState) -> Result<(), ShellError> {
        let color = if let Some(arg) = args.get(1) {
            let lower = arg.to_lowercase();
            match lower.as_str() {
                "red" => Fg::Red,
                "green" => Fg::Green,
                "blue" => Fg::Blue,
                "cyan" => Fg::Cyan,
                "yellow" => Fg::Yellow,
                "magenta" => Fg::Magenta,
                _ => Fg::Default,
            }
        } else {
            Fg::Default
        };

        _state.prompt_color = color;
        crate::color::apply_terminal_color(color);
        println!("Color changed!");
        Ok(())
    }
}


pub fn paint(s: &str, color: Fg) -> String {
    format!("{}{}{}", code(color), s, "\x1b[0m")
}


pub fn apply_terminal_color(c: Fg) {
    print!("{}", code(c));
    let _ = io::stdout().flush();
}

// #[cfg(unix)]
// pub fn name_color_by_type(path: &std::path::Path) -> String {
//     use std::os::unix::fs::PermissionsExt;
//     if let Ok(meta) = std::fs::symlink_metadata(path) {
//         if meta.is_dir() {
//             return paint(
//                 path.file_name().unwrap().to_string_lossy().as_ref(),
//                 Fg::Blue,
//             );
//         }
//         if meta.is_file() && (meta.permissions().mode() & 0o111) != 0 {
//             return paint(
//                 path.file_name().unwrap().to_string_lossy().as_ref(),
//                 Fg::Green,
//             );
//         }
//     }
//     path.file_name().unwrap().to_string_lossy().into_owned()
// }

// #[cfg(not(unix))]
// pub fn name_color_by_type(path: &std::path::Path) -> String {
//     // على ويندوز: بس لّون المجلدات كأبسط شيء
//     if let Ok(meta) = std::fs::metadata(path) {
//         if meta.is_dir() {
//             return paint(
//                 path.file_name().unwrap().to_string_lossy().as_ref(),
//                 Fg::Blue,
//             );
//         }
//     }
//     path.file_name().unwrap().to_string_lossy().into_owned()
// }

// pub fn error_red(msg: &str) -> String {
//     paint(msg, Fg::Red)
// }
