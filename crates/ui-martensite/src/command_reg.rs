//! CAD command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "draw.line", label: "Line", shortcut: Some("L") },
    Command { id: "draw.circle", label: "Circle", shortcut: Some("C") },
    Command { id: "modify.trim", label: "Trim", shortcut: Some("TR") },
    Command { id: "modify.offset", label: "Offset", shortcut: Some("O") },
];
