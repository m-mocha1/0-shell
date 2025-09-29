#[derive(Debug)]
pub enum ShellError {
    Io(std::io::Error),
    Usage(&'static str),
    NotFound(String),
}

impl From<std::io::Error> for ShellError {
    fn from(e: std::io::Error) -> Self {
        ShellError::Io(e)
    }
}
