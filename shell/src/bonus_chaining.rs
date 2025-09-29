// Command chaining with ;
pub fn chain_commands(input: &str) -> Vec<&str> {
    input.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
}
