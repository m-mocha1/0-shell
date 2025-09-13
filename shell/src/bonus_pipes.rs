// Pipes (|) feature stub
pub fn parse_pipes(input: &str) -> Vec<&str> {
    input.split('|').map(|s| s.trim()).collect()
}
