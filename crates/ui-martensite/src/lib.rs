//! Sovereign retained-mode CAD interface for CADCraft built on Martensite.

pub mod cli_prompt;
pub mod command_reg;
pub mod menus;
pub mod theme;

pub struct CadcraftApp {
    pub prompt: cli_prompt::CadCliState,
    pub ortho_mode: bool,
    pub snap_mode: bool,
    pub grid_visible: bool,
}

impl CadcraftApp {
    pub fn new() -> Self {
        Self {
            prompt: cli_prompt::CadCliState::new(),
            ortho_mode: false,
            snap_mode: true,
            grid_visible: true,
        }
    }

    pub fn toggle_ortho(&mut self) -> bool {
        self.ortho_mode = !self.ortho_mode;
        self.ortho_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cadcraft_app() {
        let mut app = CadcraftApp::new();
        assert!(!app.ortho_mode);
        assert!(app.toggle_ortho());
        assert!(app.ortho_mode);
    }
}
