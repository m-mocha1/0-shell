use crate::{Builtin, ShellState};
use crate::error::ShellError;
use crate::color::{paint, Fg, name_color_by_type};
use std::fs;
use std::path::Path;

pub struct Ls;

impl Builtin for Ls {
    fn name(&self) -> &'static str { "ls" }

    fn run(&self, argv: &[String], sh: &mut ShellState) -> Result<(), ShellError> {
        let mut show_all = false;
        let mut long_format = false;
        let mut classify = false;
        let mut path = sh.cwd.as_path();

        // Parse arguments
        for arg in &argv[1..] {
            match arg.as_str() {
                "-a" => show_all = true,
                "-l" => long_format = true,
                "-F" => classify = true,
                "-la" | "-al" => {
                    show_all = true;
                    long_format = true;
                }
                other if !other.starts_with('-') => {
                    path = Path::new(other);
                }
                _ => {
                    return Err(ShellError::Usage("usage: ls [-a] [-l] [-F] [path]"));
                }
            }
        }

        let entries = fs::read_dir(path)?;
        let mut items: Vec<_> = entries.collect::<Result<Vec<_>, _>>()?;
        items.sort_by_key(|e| e.file_name());

        if long_format {
            println!("total {}", items.len());
        }

        for entry in items {
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();

            // Skip hidden files unless -a is specified
            if !show_all && filename_str.starts_with('.') {
                continue;
            }

            let path = entry.path();
            let metadata = entry.metadata()?;

            if long_format {
                let file_type = if metadata.is_dir() { "d" } else { "-" };
                let permissions = "rwxr-xr-x"; // Simplified
                let size = metadata.len();
                
                let colored_name = name_color_by_type(&path);

                println!("{}{} 1 user user {:>8} {} {}", 
                    file_type, 
                    permissions, 
                    size,
                    "Jan 01 12:00",
                    colored_name
                );
            } else {
                let colored_name = name_color_by_type(&path);
                print!("{}  ", colored_name);
            }
        }

        if !long_format {
            println!();
        }

        Ok(())
    }
}