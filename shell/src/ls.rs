#![cfg(unix)]
use std::cmp::Ordering;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::builtin::Builtin;
use crate::error::ShellError;
use crate::repl::ShellState;

#[derive(Clone, Copy, Default)]
struct LsFlags {
    all: bool,       // -a
    long: bool,      // -l
    classify: bool,  // -F
}

pub struct Ls;

impl Builtin for Ls {
    fn name(&self) -> &'static str { "ls" }

    fn run(&self, argv: &[String], sh: &mut ShellState) -> Result<(), ShellError> {
        let (flags, paths) = parse_flags_and_paths(argv, &sh.cwd);
        let multi = paths.len() > 1;

        for (i, p) in paths.iter().enumerate() {
            let meta = match fs::symlink_metadata(p) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("ls: {}: {}", p.display(), e);
                    continue;
                }
            };

            if multi {
                if i > 0 { println!(); }
                println!("{}:", p.display());
            }

            if meta.is_dir() {
                list_dir(p, flags)?;
            } else {
                print_entry(p.file_name().unwrap_or_default().to_string_lossy().as_ref(), &meta, flags)?;
            }
        }
        Ok(())
    }
}

fn parse_flags_and_paths(argv: &[String], cwd: &Path) -> (LsFlags, Vec<PathBuf>) {
    let mut flags = LsFlags::default();
    let mut paths: Vec<PathBuf> = Vec::new();

    for a in argv {
        if a.starts_with('-') && a.len() > 1 {
            for ch in a.chars().skip(1) {
                match ch {
                    'a' => flags.all = true,
                    'l' => flags.long = true,
                    'F' => flags.classify = true,
                    _ => eprintln!("ls: unsupported flag -{}", ch),
                }
            }
        } else {
            let p = PathBuf::from(a);
            paths.push(if p.is_absolute() { p } else { cwd.join(p) });
        }
    }
    if paths.is_empty() { paths.push(cwd.to_path_buf()); }
    (flags, paths)
}

fn list_dir(dir: &Path, flags: LsFlags) -> Result<(), ShellError> {
    let mut items: Vec<(String, fs::Metadata)> = Vec::new();
    for ent in fs::read_dir(dir).map_err(Into::into)? {
        let ent = ent.map_err(Into::into)?;
        let name = ent.file_name().to_string_lossy().into_owned();
        if !flags.all && name.starts_with('.') {
            continue;
        }
        let meta = fs::symlink_metadata(ent.path()).map_err(Into::into)?;
        items.push((name, meta));
    }

    items.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, meta) in items {
        print_entry(&name, &meta, flags)?;
    }
    Ok(())
}

fn print_entry(name: &str, meta: &fs::Metadata, flags: LsFlags) -> Result<(), ShellError> {
    let mode = meta.mode();
    let display_name = if flags.classify {
        format!("{}{}", name, classify_suffix(mode))
    } else {
        name.to_string()
    };

    if flags.long {
        let perms = perms_string(mode);
        let nlink = meta.nlink();
        let uid = meta.uid();
        let gid = meta.gid();
        let size = meta.size();
        let when = format_mtime(meta.mtime());
        println!("{perms} {nlink:>2} {uid:>5} {gid:>5} {size:>8} {when} {display_name}");
    } else {
        println!("{display_name}");
    }
    Ok(())
}

/* ---------- utils ---------- */

fn file_type_char(mode: u32) -> char {
    const S_IFMT:   u32 = 0o170000;
    const S_IFSOCK: u32 = 0o140000;
    const S_IFLNK:  u32 = 0o120000;
    const S_IFREG:  u32 = 0o100000;
    const S_IFBLK:  u32 = 0o060000;
    const S_IFDIR:  u32 = 0o040000;
    const S_IFCHR:  u32 = 0o020000;
    const S_IFIFO:  u32 = 0o010000;
    match mode & S_IFMT {
        S_IFDIR => 'd',
        S_IFLNK => 'l',
        S_IFCHR => 'c',
        S_IFBLK => 'b',
        S_IFIFO => 'p',
        S_IFSOCK => 's',
        S_IFREG => '-',
        _ => '?',
    }
}

fn perms_string(mode: u32) -> String {
    let t = file_type_char(mode);
    let part = |r, w, x| {
        let r = if (mode & r) != 0 { 'r' } else { '-' };
        let w = if (mode & w) != 0 { 'w' } else { '-' };
        let x = if (mode & x) != 0 { 'x' } else { '-' };
        format!("{r}{w}{x}")
    };
    format!(
        "{}{}{}{}",
        t,
        part(0o400, 0o200, 0o100),
        part(0o040, 0o020, 0o010),
        part(0o004, 0o002, 0o001),
    )
}

fn is_executable(mode: u32) -> bool { (mode & 0o111) != 0 }

fn classify_suffix(mode: u32) -> &'static str {
    match file_type_char(mode) {
        'd' => "/",
        'l' => "@",
        _ if is_executable(mode) => "*",
        _ => "",
    }
}

fn format_mtime(mtime_secs: i64) -> String {
    use std::time::Duration;
    let dt = UNIX_EPOCH + Duration::from_secs(mtime_secs as u64);
    let datetime: chrono_like::DateTime = dt.into(); // helper أدناه
    format!("{:04}-{:02}-{:02} {:02}:{:02}",
        datetime.year, datetime.month, datetime.day, datetime.hour, datetime.minute)
}

mod chrono_like {
    use std::time::{Duration, SystemTime};

    #[derive(Clone, Copy)]
    pub struct DateTime { pub year:i32, pub month:u8, pub day:u8, pub hour:u8, pub minute:u8 }

    impl From<SystemTime> for DateTime {
        fn from(t: SystemTime) -> Self {
            const SECS_PER_MIN: i64 = 60;
            const SECS_PER_HOUR: i64 = 3600;
            const SECS_PER_DAY: i64 = 86400;

            let dur = t.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
            let mut secs = dur.as_secs() as i64;

            let days = secs / SECS_PER_DAY;
            let mut rem = secs % SECS_PER_DAY;
            if rem < 0 { rem += SECS_PER_DAY; }

            let hour = (rem / SECS_PER_HOUR) as u8;
            rem %= SECS_PER_HOUR;
            let minute = (rem / SECS_PER_MIN) as u8;

            let (year, month, day) = days_to_ymd(days);
            DateTime { year, month, day, hour, minute }
        }
    }

    pub fn days_to_ymd(days_since_1970: i64) -> (i32, u8, u8) {
        let mut z = days_since_1970 + 719468;
        let era = (z >= 0).then(|| z).unwrap_or(z - 146096) / 146097;
        let doe = z - era * 146097;                         
        let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365; 
        let y = yoe + era * 400;
        let doy = doe - (365*yoe + yoe/4 - yoe/100);
        let mp = (5*doy + 2)/153;                           
        let d = doy - (153*mp + 2)/5 + 1;                   
        let m = mp + if mp < 10 {3} else {-9};              
        let year = (y + (m <= 2) as i64) as i32;
        (year, m as u8, d as u8)
    }
}
