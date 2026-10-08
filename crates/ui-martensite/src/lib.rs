//! Sovereign retained-mode CAD interface for CADCraft built on the Martensite GUI engine.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod cli_prompt;
pub mod command_reg;
pub mod menus;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use std::sync::{Arc, Mutex};

use cadcraft_engine::Engine;

/// Application state container managing the Martensite GUI pipeline.
pub struct CadcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: cadcraft_engine::Tool,
    /// Command-line HUD state: the AutoCAD-style prompt at the foot of the canvas.
    pub prompt: cli_prompt::CadCliState,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    pub ortho_mode: bool,
    pub snap_mode: bool,
    pub grid_visible: bool,
    pub is_dirty: bool,
}

impl CadcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::dark_neutral(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: cadcraft_engine::Tool::Select,
            prompt: cli_prompt::CadCliState::new(),
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            ortho_mode: false,
            snap_mode: true,
            grid_visible: true,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: cadcraft_engine::Tool) {
        self.active_tool = tool;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.01, 64.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    pub fn toggle_ortho(&mut self) -> bool {
        self.ortho_mode = !self.ortho_mode;
        self.ortho_mode
    }

    pub fn toggle_snap(&mut self) -> bool {
        self.snap_mode = !self.snap_mode;
        self.snap_mode
    }

    pub fn toggle_grid(&mut self) -> bool {
        self.grid_visible = !self.grid_visible;
        self.grid_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = CadcraftApp::new(engine);
        assert_eq!(app.active_tool, cadcraft_engine::Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(!app.ortho_mode);
        assert!(app.snap_mode);
        assert!(app.grid_visible);
        assert!(!app.is_dirty);
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = CadcraftApp::new(engine);

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.0001);
        assert_eq!(app.zoom_level, 0.01);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 64.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = CadcraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_toggles() {
        let engine = Engine::new();
        let mut app = CadcraftApp::new(engine);

        assert!(!app.ortho_mode);
        assert!(app.toggle_ortho());
        assert!(app.ortho_mode);
        assert!(!app.toggle_ortho());

        assert!(app.snap_mode);
        assert!(!app.toggle_snap());
        assert!(!app.snap_mode);

        assert!(app.grid_visible);
        assert!(!app.toggle_grid());
        assert!(!app.grid_visible);
    }
}
