// Command chaining with ;
// Parse and execute multiple commands separated by ';'

pub fn chain_commands(input: &str) -> Vec<&str> {
    input.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
}
