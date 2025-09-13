// Command history feature stub
pub struct History {
    pub commands: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        History { commands: Vec::new() }
    }
    pub fn add(&mut self, cmd: String) {
        self.commands.push(cmd);
    }
    pub fn get(&self, idx: usize) -> Option<&String> {
        self.commands.get(idx)
    }
}
