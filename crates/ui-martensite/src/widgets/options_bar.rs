//! Options bar widget adapting dynamically to the active tool.
//!
//! Holds the numeric parameters the running draw/modify command would otherwise
//! prompt for on the command line (radius, offset distance, chamfer distance…).

use crate::widgets::scrubby_input::ScrubbyInputWidget;
use cadcraft_engine::Tool;

pub struct OptionsBarWidget {
    pub active_tool: Tool,
    pub radius: ScrubbyInputWidget,
    pub offset_distance: ScrubbyInputWidget,
    pub chamfer_distance: ScrubbyInputWidget,
    pub ortho_mode: bool,
    pub snap_mode: bool,
}

impl OptionsBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Line,
            radius: ScrubbyInputWidget::new("Radius", 10.0, 0.0, 1_000_000.0, "u"),
            offset_distance: ScrubbyInputWidget::new("Offset", 1.0, 0.0, 1_000_000.0, "u"),
            chamfer_distance: ScrubbyInputWidget::new("Distance", 0.5, 0.0, 1_000_000.0, "u"),
            ortho_mode: false,
            snap_mode: true,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn toggle_ortho(&mut self) -> bool {
        self.ortho_mode = !self.ortho_mode;
        self.ortho_mode
    }

    pub fn toggle_snap(&mut self) -> bool {
        self.snap_mode = !self.snap_mode;
        self.snap_mode
    }
}

impl Default for OptionsBarWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_bar_defaults() {
        let mut bar = OptionsBarWidget::new();
        assert_eq!(bar.active_tool, Tool::Line);
        assert_eq!(bar.radius.value, 10.0);
        assert!(!bar.ortho_mode);
        assert!(bar.snap_mode);

        bar.set_tool(Tool::Circle);
        assert_eq!(bar.active_tool, Tool::Circle);

        assert!(bar.toggle_ortho());
        assert!(bar.ortho_mode);
        assert!(!bar.toggle_ortho());
    }
}
