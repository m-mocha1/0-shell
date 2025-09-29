// src/cp.rs
use crate::{Builtin, ShellState};
use crate::error::ShellError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Cp;

impl Builtin for Cp {
    fn name(&self) -> &'static str { "cp" }

    fn run(&self, argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        if argv.len() != 3 {
            return Err(ShellError::Usage("usage: cp SRC DST"));
        }
        let src = Path::new(&argv[1]);
        let dst_input = Path::new(&argv[2]);

        if !src.is_file() {
            return Err(ShellError::Usage("cp currently supports file->file or file->dir"));
        }

        // إذا كان DST مجلد، كوّن المسار: dst/filename
        let dst: PathBuf = if dst_input.is_dir() {
            let fname = src.file_name().ok_or_else(|| ShellError::Usage("invalid SRC filename"))?;
            dst_input.join(fname)
        } else {
            dst_input.to_path_buf()
        };

        // انسخ البايتات
        fs::copy(&src, &dst)?;

        // (Bonus) نسخ الصلاحيات/المود لو أمكن
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = fs::metadata(&src) {
                let perm = meta.permissions();
                let mode = perm.mode();
                let mut newperm = fs::metadata(&dst)?.permissions();
                newperm.set_mode(mode);
                let _ = fs::set_permissions(&dst, newperm);
            }
        }

        Ok(())
    }
}
