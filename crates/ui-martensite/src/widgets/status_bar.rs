//! Status bar widget: the drafting-aid toggles along the foot of the canvas
//! (ORTHO, SNAP, GRID, OSNAP, POLAR, lineweight display).

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StatusBarWidget {
    pub ortho: bool,
    pub snap: bool,
    pub grid: bool,
    pub osnap: bool,
    pub polar: bool,
    pub lineweight_display: bool,
}

impl StatusBarWidget {
    pub fn new() -> Self {
        Self { ortho: false, snap: true, grid: true, osnap: true, polar: true, lineweight_display: false }
    }

    /// Toggle a drafting aid by its status-bar name; returns the new state,
    /// or `None` for an unknown aid name.
    pub fn toggle(&mut self, aid: &str) -> Option<bool> {
        let flag = match aid {
            "ortho" => &mut self.ortho,
            "snap" => &mut self.snap,
            "grid" => &mut self.grid,
            "osnap" => &mut self.osnap,
            "polar" => &mut self.polar,
            "lineweight" => &mut self.lineweight_display,
            _ => return None,
        };
        *flag = !*flag;
        Some(*flag)
    }
}

impl Default for StatusBarWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_toggles() {
        let mut bar = StatusBarWidget::new();
        assert!(!bar.ortho);
        assert!(bar.snap);
        assert!(bar.grid);
        assert!(bar.osnap);
        assert!(bar.polar);
        assert!(!bar.lineweight_display);

        assert_eq!(bar.toggle("ortho"), Some(true));
        assert!(bar.ortho);
        assert_eq!(bar.toggle("snap"), Some(false));
        assert!(!bar.snap);
        assert_eq!(bar.toggle("lineweight"), Some(true));
        assert!(bar.lineweight_display);

        assert_eq!(bar.toggle("nonexistent"), None);
    }
}
