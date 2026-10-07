//! CAD menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "Draw", items: &["draw.line", "draw.circle"] },
    MenuCategory { title: "Modify", items: &["modify.trim", "modify.offset"] },
];
