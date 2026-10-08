//! CAD command prompt HUD state machine.

pub struct CadCliState {
    pub current_input: String,
    pub history: Vec<String>,
}

impl CadCliState {
    pub fn new() -> Self {
        Self { current_input: String::new(), history: Vec::new() }
    }

    pub fn submit_command(&mut self) -> Option<String> {
        let cmd = self.current_input.trim().to_uppercase();
        if !cmd.is_empty() {
            self.history.push(cmd.clone());
            self.current_input.clear();
            Some(cmd)
        } else {
            None
        }
    }
}

impl Default for CadCliState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_prompt() {
        let mut cli = CadCliState::new();
        cli.current_input = "line".to_string();
        let cmd = cli.submit_command();
        assert_eq!(cmd, Some("LINE".to_string()));
        assert_eq!(cli.history.len(), 1);
    }
}
