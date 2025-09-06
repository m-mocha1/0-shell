use crate::{Builtin, ShellState};
use crate::error::ShellError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Mv;

impl Builtin for Mv {
    fn name(&self) -> &'static str { "mv" }

    fn run(&self, argv: &[String], _sh: &mut ShellState) -> Result<(), ShellError> {
        if argv.len() != 3 {
            return Err(ShellError::Usage("usage: mv SRC DST"));
        }
        let src = Path::new(&argv[1]);
        let dst_input = Path::new(&argv[2]);

        if !src.exists() {
            return Err(ShellError::NotFound(argv[1].clone()));
        }

        // file->dir: ألحق الاسم
        let dst: PathBuf = if dst_input.is_dir() {
            let fname = src.file_name().ok_or_else(|| ShellError::Usage("invalid SRC filename"))?;
            dst_input.join(fname)
        } else {
            dst_input.to_path_buf()
        };

        match fs::rename(&src, &dst) {
            Ok(_) => Ok(()),
            Err(e) => {
                if is_cross_fs(&e) {
                    // fallback: copy + remove (ندعم الملفات فقط هنا)
                    if src.is_file() {
                        fs::copy(&src, &dst)?;
                        fs::remove_file(&src)?;
                        Ok(())
                    } else {
                        Err(ShellError::Usage("mv across filesystems currently supports files only"))
                    }
                } else {
                    Err(ShellError::Io(e))
                }
            }
        }
    }
}

// === Cross-FS detection (portable) ===
#[cfg(unix)]
fn is_cross_fs(e: &std::io::Error) -> bool {
    matches!(e.raw_os_error(), Some(libc::EXDEV))
}

#[cfg(windows)]
fn is_cross_fs(e: &std::io::Error) -> bool {
    // ERROR_NOT_SAME_DEVICE = 17
    matches!(e.raw_os_error(), Some(17))
}

#[cfg(not(any(unix, windows)))]
fn is_cross_fs(_e: &std::io::Error) -> bool { false }
