// Pipes (|) feature stub
// Parse and execute piped commands

pub fn parse_pipes(input: &str) -> Vec<&str> {
    input.split('|').map(|s| s.trim()).collect()
}
