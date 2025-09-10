// #![cfg(unix)]
// use std::cmp::Ordering;
// use std::ffi::OsString;
// use std::fs;
// use std::io::{self, Write};
// use std::os::unix::fs::MetadataExt;
// use std::path::{Path, PathBuf};
// use std::time::{SystemTime, UNIX_EPOCH};

// use crate::builtin::Builtin;
// use crate::error::ShellError;
// use crate::repl::ShellState;

// #[derive(Clone, Copy, Default)]
// struct LsFlags {
//     all: bool,       // -a
//     long: bool,      // -l
//     classify: bool,  // -F
// }

// pub struct Ls;

// impl Builtin for Ls {
//     fn name(&self) -> &'static str { "ls" }

//     fn run(&self, argv: &[String], sh: &mut ShellState) -> Result<(), ShellError> {
//         let (flags, paths) = parse_flags_and_paths(argv, &sh.cwd);
//         let multi = paths.len() > 1;

//         for (i, p) in paths.iter().enumerate() {
//             let meta = match fs::symlink_metadata(p) {
//                 Ok(m) => m,
//                 Err(e) => {
//                     eprintln!("ls: {}: {}", p.display(), e);
//                     continue;
//                 }
//             };

//             if multi {
//                 if i > 0 { println!(); }
//                 println!("{}:", p.display());
//             }

//             if meta.is_dir() {
//                 list_dir(p, flags)?;
//             } else {
//                 print_entry(p.file_name().unwrap_or_default().to_string_lossy().as_ref(), &meta, flags)?;
//             }
//         }
//         Ok(())
//     }
// }

// fn parse_flags_and_paths(argv: &[String], cwd: &Path) -> (LsFlags, Vec<PathBuf>) {
//     let mut flags = LsFlags::default();
//     let mut paths: Vec<PathBuf> = Vec::new();

//     for a in argv {
//         if a.starts_with('-') && a.len() > 1 {
//             for ch in a.chars().skip(1) {
//                 match ch {
//                     'a' => flags.all = true,
//                     'l' => flags.long = true,
//                     'F' => flags.classify = true,
//                     _ => eprintln!("ls: unsupported flag -{}", ch),
//                 }
//             }
//         } else {
//             let p = PathBuf::from(a);
//             paths.push(if p.is_absolute() { p } else { cwd.join(p) });
//         }
//     }
//     if paths.is_empty() { paths.push(cwd.to_path_buf()); }
//     (flags, paths)
// }

// fn list_dir(dir: &Path, flags: LsFlags) -> Result<(), ShellError> {
//     let mut items: Vec<(String, fs::Metadata)> = Vec::new();
//     for ent in fs::read_dir(dir).map_err(Into::into)? {
//         let ent = ent.map_err(Into::into)?;
//         let name = ent.file_name().to_string_lossy().into_owned();
//         if !flags.all && name.starts_with('.') {
//             continue;
//         }
//         let meta = fs::symlink_metadata(ent.path()).map_err(Into::into)?;
//         items.push((name, meta));
//     }

//     items.sort_by(|a, b| a.0.cmp(&b.0));
//     for (name, meta) in items {
//         print_entry(&name, &meta, flags)?;
//     }
//     Ok(())
// }

// fn print_entry(name: &str, meta: &fs::Metadata, flags: LsFlags) -> Result<(), ShellError> {
//     let mode = meta.mode();
//     let display_name = if flags.classify {
//         format!("{}{}", name, classify_suffix(mode))
//     } else {
//         name.to_string()
//     };

//     if flags.long {
//         let perms = perms_string(mode);
//         let nlink = meta.nlink();
//         let uid = meta.uid();
//         let gid = meta.gid();
//         let size = meta.size();
//         let when = format_mtime(meta.mtime());
//         println!("{perms} {nlink:>2} {uid:>5} {gid:>5} {size:>8} {when} {display_name}");
//     } else {
//         println!("{display_name}");
//     }
//     Ok(())
// }

// /* ---------- utils ---------- */

// fn file_type_char(mode: u32) -> char {
//     const S_IFMT:   u32 = 0o170000;
//     const S_IFSOCK: u32 = 0o140000;
//     const S_IFLNK:  u32 = 0o120000;
//     const S_IFREG:  u32 = 0o100000;
//     const S_IFBLK:  u32 = 0o060000;
//     const S_IFDIR:  u32 = 0o040000;
//     const S_IFCHR:  u32 = 0o020000;
//     const S_IFIFO:  u32 = 0o010000;
//     match mode & S_IFMT {
//         S_IFDIR => 'd',
//         S_IFLNK => 'l',
//         S_IFCHR => 'c',
//         S_IFBLK => 'b',
//         S_IFIFO => 'p',
//         S_IFSOCK => 's',
//         S_IFREG => '-',
//         _ => '?',
//     }
// }

// fn perms_string(mode: u32) -> String {
//     let t = file_type_char(mode);
//     let part = |r, w, x| {
//         let r = if (mode & r) != 0 { 'r' } else { '-' };
//         let w = if (mode & w) != 0 { 'w' } else { '-' };
//         let x = if (mode & x) != 0 { 'x' } else { '-' };
//         format!("{r}{w}{x}")
//     };
//     format!(
//         "{}{}{}{}",
//         t,
//         part(0o400, 0o200, 0o100),
//         part(0o040, 0o020, 0o010),
//         part(0o004, 0o002, 0o001),
//     )
// }

// fn is_executable(mode: u32) -> bool { (mode & 0o111) != 0 }

// fn classify_suffix(mode: u32) -> &'static str {
//     match file_type_char(mode) {
//         'd' => "/",
//         'l' => "@",
//         _ if is_executable(mode) => "*",
//         _ => "",
//     }
// }

// fn format_mtime(mtime_secs: i64) -> String {
//     use std::time::Duration;
//     let dt = UNIX_EPOCH + Duration::from_secs(mtime_secs as u64);
//     let datetime: chrono_like::DateTime = dt.into(); // helper أدناه
//     format!("{:04}-{:02}-{:02} {:02}:{:02}",
//         datetime.year, datetime.month, datetime.day, datetime.hour, datetime.minute)
// }

// mod chrono_like {
//     use std::time::{Duration, SystemTime};

//     #[derive(Clone, Copy)]
//     pub struct DateTime { pub year:i32, pub month:u8, pub day:u8, pub hour:u8, pub minute:u8 }

//     impl From<SystemTime> for DateTime {
//         fn from(t: SystemTime) -> Self {
//             const SECS_PER_MIN: i64 = 60;
//             const SECS_PER_HOUR: i64 = 3600;
//             const SECS_PER_DAY: i64 = 86400;

//             let dur = t.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
//             let mut secs = dur.as_secs() as i64;

//             let days = secs / SECS_PER_DAY;
//             let mut rem = secs % SECS_PER_DAY;
//             if rem < 0 { rem += SECS_PER_DAY; }

//             let hour = (rem / SECS_PER_HOUR) as u8;
//             rem %= SECS_PER_HOUR;
//             let minute = (rem / SECS_PER_MIN) as u8;

//             let (year, month, day) = days_to_ymd(days);
//             DateTime { year, month, day, hour, minute }
//         }
//     }

//     pub fn days_to_ymd(days_since_1970: i64) -> (i32, u8, u8) {
//         let mut z = days_since_1970 + 719468;
//         let era = (z >= 0).then(|| z).unwrap_or(z - 146096) / 146097;
//         let doe = z - era * 146097;                         
//         let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365; 
//         let y = yoe + era * 400;
//         let doy = doe - (365*yoe + yoe/4 - yoe/100);
//         let mp = (5*doy + 2)/153;                           
//         let d = doy - (153*mp + 2)/5 + 1;                   
//         let m = mp + if mp < 10 {3} else {-9};              
//         let year = (y + (m <= 2) as i64) as i32;
//         (year, m as u8, d as u8)
//     }
// }
/////////////////////////////////////////////////////////
/// for windows 
/// 
/// use std::fs;
use std::path::{Path, PathBuf};
use std::time::{UNIX_EPOCH, Duration};
use std::fs;
use crate::builtin::Builtin;
use crate::error::ShellError;
use crate::repl::ShellState;
use crate::color::{paint, Fg};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

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
                Err(e) => { eprintln!("ls: {}: {}", p.display(), e); continue; }
            };

            if multi {
                if i > 0 { println!(); }
                println!("{}:", p.display());
            }

            if meta.is_dir() {
                list_dir(p, flags)?;
            } else {
                let name = p.file_name().unwrap_or_default().to_string_lossy().into_owned();
                print_entry(Some(p), &name, &meta, flags)?;
            }
        }
        Ok(())
    }
}

fn parse_flags_and_paths(argv: &[String], cwd: &Path) -> (LsFlags, Vec<PathBuf>) {
    let mut flags = LsFlags::default();
    let mut paths: Vec<PathBuf> = Vec::new();

    // ✅ مهم: تخطّي argv[0] لأنه اسم الأمر ("ls")
    for a in argv.iter().skip(1) {
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
    let mut items: Vec<(String, fs::Metadata, PathBuf)> = Vec::new();

    // ✅ أضف الإدخالات الخاصة عند -a
    if flags.all {
        // "."
        if let Ok(meta_dot) = fs::symlink_metadata(dir) {
            items.push((".".to_string(), meta_dot, dir.to_path_buf()));
        }
        // ".."
        let parent = dir.join("..");
        if let Ok(meta_ddot) = fs::symlink_metadata(&parent) {
            items.push(("..".to_string(), meta_ddot, parent));
        }
    }

    // بقية العناصر الفعلية داخل المجلد
    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let name = ent.file_name().to_string_lossy().into_owned();
        if !flags.all && name.starts_with('.') { continue; }
        let full = ent.path();
        let meta = fs::symlink_metadata(&full)?;
        items.push((name, meta, full));
    }

    items.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    for (name, meta, full) in items {
        print_entry(Some(&full), &name, &meta, flags)?;
    }
    Ok(())
}


/// اطبع مدخل واحد؛ لو symlink وأنتِ في -l، اعرض "name -> target"
fn print_entry(full_path: Option<&Path>, name: &str, meta: &fs::Metadata, flags: LsFlags)
    -> Result<(), ShellError>
{
    let colored_name = colorize_name(name, meta);
    let display_name = if flags.classify {
        format!("{}{}", colored_name, classify_suffix(name, meta))
    } else {
        colored_name
    };

    if flags.long {
        let (perms, nlink, owner, group, size, mtime_secs) = long_fields(name, meta);
        let when = format_mtime(mtime_secs);
        // إن كان Symlink، حاول اقرأي الهدف
        if is_symlink(meta) {
            if let Some(p) = full_path {
                if let Ok(target) = fs::read_link(p) {
                    let tgt = target.to_string_lossy();
                    println!("{perms} {nlink:>2} {owner:>8} {group:>8} {size:>8} {when} {display_name} -> {tgt}");
                    return Ok(());
                }
            }
        }
        println!("{perms} {nlink:>2} {owner:>8} {group:>8} {size:>8} {when} {display_name}");
    } else {
        println!("{display_name}");
    }
    Ok(())
}

/* ---------- classify (-F) ---------- */

fn classify_suffix(name: &str, meta: &fs::Metadata) -> &'static str {
    if meta.is_dir() { return "/"; }
    if is_symlink(meta) { return "@"; }
    if is_executable(name, meta) { return "*"; }
    ""
}

#[cfg(unix)]
fn is_executable(_name: &str, meta: &fs::Metadata) -> bool {
    #[allow(deprecated)]
    { (meta.mode() & 0o111) != 0 }
}

#[cfg(not(unix))]
fn is_executable(name: &str, _meta: &fs::Metadata) -> bool {
    // Windows: نحدد التنفيذي من PATHEXT
    let ext = Path::new(name).extension()
        .map(|s| s.to_string_lossy().to_uppercase())
        .unwrap_or_default();
    if ext.is_empty() { return false; }
    let pathext = std::env::var("PATHEXT")
        .unwrap_or(".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC;.PS1;.MSI;.MSP".into())
        .to_uppercase();
    pathext.split(';').any(|e| e.trim_start_matches('.').eq(ext.trim_start_matches('.')))
}

fn is_symlink(meta: &fs::Metadata) -> bool {
    meta.file_type().is_symlink()
}

/* ---------- long listing fields ---------- */

#[cfg(unix)]
fn long_fields(_name: &str, meta: &fs::Metadata) -> (String, u64, String, String, u64, i64) {
    let mode = meta.mode();
    let perms = perms_string_unix(mode);
    let nlink = meta.nlink();
    let owner = meta.uid().to_string();
    let group = meta.gid().to_string();
    let size = meta.size();
    let mtime_secs = meta.mtime();
    (perms, nlink, owner, group, size, mtime_secs)
}

#[cfg(not(unix))]
fn long_fields(name: &str, meta: &fs::Metadata) -> (String, u64, String, String, u64, i64) {
    let perms = perms_string_win(name, meta);
    let nlink = 1; // تقريب
    let owner = "-".to_string();
    let group = "-".to_string();
    let size = meta.len();
    let mtime_secs = meta.modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    (perms, nlink, owner, group, size, mtime_secs)
}

/* ---------- perms string ---------- */

#[cfg(unix)]
fn perms_string_unix(mode: u32) -> String {
    let t = file_type_char_unix(mode);
    let tri = |r, w, x| {
        let r = if (mode & r) != 0 { 'r' } else { '-' };
        let w = if (mode & w) != 0 { 'w' } else { '-' };
        let x = if (mode & x) != 0 { 'x' } else { '-' };
        format!("{r}{w}{x}")
    };
    format!("{}{}{}{}", t, tri(0o400,0o200,0o100), tri(0o040,0o020,0o010), tri(0o004,0o002,0o001))
}

#[cfg(unix)]
fn file_type_char_unix(mode: u32) -> char {
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

#[cfg(not(unix))]
fn perms_string_win(name: &str, meta: &fs::Metadata) -> String {
    let t = if meta.is_dir() { 'd' } else if meta.file_type().is_symlink() { 'l' } else { '-' };
    let r = 'r';
    let w = if meta.permissions().readonly() { '-' } else { 'w' };
    let x = if is_executable(name, meta) { 'x' } else { '-' };
    // نكرّر المجموعة ثلاث مرات (تقريب rwx للمستخدم/المجموعة/الآخرين)
    format!("{t}{r}{w}{x}{r}{w}{x}{r}{w}{x}")
}

/* ---------- time formatting ---------- */

fn format_mtime(secs_since_epoch: i64) -> String {
    // YYYY-MM-DD HH:MM (UTC-ish)
    let dt = UNIX_EPOCH + Duration::from_secs(secs_since_epoch.max(0) as u64);
    let datetime: chrono_like::DateTime = dt.into();
    format!("{:04}-{:02}-{:02} {:02}:{:02}",
        datetime.year, datetime.month, datetime.day, datetime.hour, datetime.minute)
}

/* --- tiny no-deps date helper --- */
mod chrono_like {
    use std::time::SystemTime;
    #[derive(Clone, Copy)] pub struct DateTime { pub year:i32, pub month:u8, pub day:u8, pub hour:u8, pub minute:u8 }
    impl From<SystemTime> for DateTime {
        fn from(t: SystemTime) -> Self {
            const MIN:i64=60; const H:i64=3600; const D:i64=86400;
            let Ok(dur)=t.duration_since(SystemTime::UNIX_EPOCH) else { return DateTime{year:1970,month:1,day:1,hour:0,minute:0}; };
            let secs = dur.as_secs() as i64;
            let days = secs / D; let mut rem = secs % D;
            if rem < 0 { rem += D; }
            let hour=(rem / H) as u8; rem%=H; let minute=(rem / MIN) as u8;
            let (year,month,day)=days_to_ymd(days);
            DateTime{year,month,day,hour,minute}
        }
    }
    pub fn days_to_ymd(days_since_1970: i64) -> (i32,u8,u8){
        let  z = days_since_1970 + 719468;
        let era = (if z>=0 {z} else {z-146096}) / 146097;
        let doe = z - era*146097;
        let yoe = (doe - doe/1460 + doe/36524 - doe/146096)/365;
        let y = yoe + era*400;
        let doy = doe - (365*yoe + yoe/4 - yoe/100);
        let mp = (5*doy + 2)/153;
        let d = doy - (153*mp + 2)/5 + 1;
        let m = mp + if mp < 10 {3} else {-9};
        let year = (y + (m <= 2) as i64) as i32;
        (year, m as u8, d as u8)
    }
}

fn colorize_name(name: &str, meta: &fs::Metadata) -> String {
    if meta.is_dir() {
        paint(name, Fg::Blue)
    } else if is_symlink(meta) {
        paint(name, Fg::Cyan)
    } else if is_executable(name, meta) {
        paint(name, Fg::Green)
    } else {
        name.to_string()
    }
}