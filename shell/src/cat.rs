// src/cat.rs
use crate::{Builtin, ShellState};
use crate::error::ShellError;
use std::fs::File;
use std::io::{self, Read, Write};

pub struct Cat;

impl Builtin for Cat {
    fn name(&self) -> &'static str { "cat" }

    fn run(&self, argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        if argv.len() < 2 {
            // cat بدون ملفات: نستقبل من الـ stdin ونطبع كما هو
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            io::stdout().write_all(&buf)?;
            return Ok(());
        }

        for arg in &argv[1..] {
            if arg == "-" {
                let mut buf = Vec::new();
                io::stdin().read_to_end(&mut buf)?;
                io::stdout().write_all(&buf)?;
                continue;
            }

            match File::open(arg) {
                Ok(mut f) => {
                    let mut buf = Vec::new();
                    if let Err(e) = f.read_to_end(&mut buf) {
                        eprintln!("cat: {}: {}", arg, e);
                        continue;
                    }
                    if let Err(e) = io::stdout().write_all(&buf) {
                        eprintln!("cat: write error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("cat: {}: {}", arg, e);
                    // لا نعمل return؛ نكمل للملف التالي
                }
            }
        }
        Ok(())
    }
}
