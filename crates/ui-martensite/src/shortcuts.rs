//! Keystroke state machine providing AutoCAD-style tool ergonomics.
//!
//! Tool-strip keys pick a drawing or modify tool directly; holding `Space`
//! spring-loads Pan and holding `Z` spring-loads Zoom Window, both restoring
//! the previous tool on release.

use cadcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Pan)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::ZoomWindow)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Tool-strip single-key mnemonics
            "v" | "V" => Some(Tool::Select),
            "l" | "L" => Some(Tool::Line),
            "p" | "P" => Some(Tool::Polyline),
            "c" | "C" => Some(Tool::Circle),
            "a" | "A" => Some(Tool::Arc),
            "m" | "M" => Some(Tool::Move),
            "o" | "O" => Some(Tool::Copy),
            "t" | "T" => Some(Tool::Trim),
            "e" | "E" => Some(Tool::Extend),
            "h" | "H" => Some(Tool::Pan),
            "x" | "X" => Some(Tool::ZoomExtents),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

impl Default for KeyboardEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", Tool::Line), Some(Tool::Select));
        assert_eq!(k.on_key_down("L", Tool::Select), Some(Tool::Line));
        assert_eq!(k.on_key_down("p", Tool::Select), Some(Tool::Polyline));
        assert_eq!(k.on_key_down("c", Tool::Select), Some(Tool::Circle));
        assert_eq!(k.on_key_down("a", Tool::Select), Some(Tool::Arc));
        assert_eq!(k.on_key_down("m", Tool::Select), Some(Tool::Move));
        assert_eq!(k.on_key_down("o", Tool::Select), Some(Tool::Copy));
        assert_eq!(k.on_key_down("t", Tool::Select), Some(Tool::Trim));
        assert_eq!(k.on_key_down("e", Tool::Select), Some(Tool::Extend));
        assert_eq!(k.on_key_down("h", Tool::Select), Some(Tool::Pan));
        assert_eq!(k.on_key_down("x", Tool::Select), Some(Tool::ZoomExtents));
    }

    #[test]
    fn test_spring_loaded_pan_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Line;

        // Press Space: temporary Pan
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Pan));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Pan), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_window_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Circle;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::ZoomWindow));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}
