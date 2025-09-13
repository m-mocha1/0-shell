// I/O redirection (>, <) feature stub
pub struct Redirection {
    pub input: Option<String>,
    pub output: Option<String>,
}

pub fn parse_redirection(_input: &str) -> Redirection {
    Redirection { input: None, output: None }
}
