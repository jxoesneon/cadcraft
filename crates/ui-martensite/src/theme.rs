//! CAD drafting theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub viewport_bg: Color,
    pub crosshair: Color,
    pub grid_dot: Color,
}

impl Theme {
    pub fn drafting_dark() -> Self {
        Self {
            viewport_bg: Color(28, 30, 34),
            crosshair: Color(255, 255, 255),
            grid_dot: Color(60, 65, 75),
        }
    }
}
