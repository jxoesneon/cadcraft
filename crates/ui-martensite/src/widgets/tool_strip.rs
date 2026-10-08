//! Tool strip widget: ribbon-style tool palette with entity property chips.

use cadcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Select, alternatives: &[] },
    ToolSlot { primary: Tool::Line, alternatives: &[Tool::Polyline] },
    ToolSlot { primary: Tool::Circle, alternatives: &[Tool::Arc] },
    ToolSlot { primary: Tool::Move, alternatives: &[Tool::Copy] },
    ToolSlot { primary: Tool::Trim, alternatives: &[Tool::Extend] },
    ToolSlot { primary: Tool::Pan, alternatives: &[] },
    ToolSlot { primary: Tool::ZoomWindow, alternatives: &[Tool::ZoomExtents] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub double_column: bool,
    /// Current entity colour (ByLayer default: white on the dark canvas).
    pub current_color: [u8; 4],
    /// Current layer name shown on the layer chip.
    pub current_layer: String,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Select,
            double_column: false,
            current_color: [255, 255, 255, 255], // ByLayer on a dark canvas
            current_layer: "0".to_string(),
        }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    pub fn set_current_color(&mut self, color: [u8; 4]) {
        self.current_color = color;
    }

    pub fn set_current_layer(&mut self, name: impl Into<String>) {
        self.current_layer = name.into();
    }
}

impl Default for ToolStripWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Select);
        assert!(!strip.double_column);
        assert_eq!(strip.current_color, [255, 255, 255, 255]);
        assert_eq!(strip.current_layer, "0");

        strip.set_current_color([255, 0, 0, 255]);
        assert_eq!(strip.current_color, [255, 0, 0, 255]);

        strip.set_current_layer("Walls");
        assert_eq!(strip.current_layer, "Walls");

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());
    }

    #[test]
    fn test_tool_slots_cover_every_tool() {
        use std::collections::HashSet;
        let all = [
            Tool::Select,
            Tool::Line,
            Tool::Polyline,
            Tool::Circle,
            Tool::Arc,
            Tool::Move,
            Tool::Copy,
            Tool::Trim,
            Tool::Extend,
            Tool::Pan,
            Tool::ZoomWindow,
            Tool::ZoomExtents,
        ];
        let covered: HashSet<Tool> = TOOL_SLOTS.iter().flat_map(|s| std::iter::once(s.primary).chain(s.alternatives.iter().copied())).collect();
        for t in all {
            assert!(covered.contains(&t), "Tool::{t:?} missing from TOOL_SLOTS");
        }
    }
}
