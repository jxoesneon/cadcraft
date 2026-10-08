//! Decoupled command catalog and taxonomy for CADCraft.
//!
//! The `id`s are the engine's command ids (AutoCAD command names in lower case);
//! `default_shortcut` is the typed command alias and `secondary_shortcut` the
//! keyboard accelerator where AutoCAD defines one.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Draw,
    Modify,
    Layers,
    View,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec { id: "new", label: "New…", category: CommandCategory::File, default_shortcut: Some("Ctrl+N"), secondary_shortcut: None },
    CommandSpec { id: "open", label: "Open…", category: CommandCategory::File, default_shortcut: Some("Ctrl+O"), secondary_shortcut: None },
    CommandSpec { id: "qsave", label: "Save", category: CommandCategory::File, default_shortcut: Some("Ctrl+S"), secondary_shortcut: None },
    CommandSpec {
        id: "saveas",
        label: "Save As…",
        category: CommandCategory::File,
        default_shortcut: Some("Ctrl+Shift+S"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "close", label: "Close", category: CommandCategory::File, default_shortcut: Some("Ctrl+F4"), secondary_shortcut: None },
    // Edit
    CommandSpec { id: "undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+Z"), secondary_shortcut: Some("U") },
    CommandSpec { id: "redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+Y"), secondary_shortcut: None },
    CommandSpec { id: "cutclip", label: "Cut", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+X"), secondary_shortcut: None },
    CommandSpec { id: "copyclip", label: "Copy", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+C"), secondary_shortcut: None },
    CommandSpec {
        id: "copybase",
        label: "Copy with Base Point",
        category: CommandCategory::Edit,
        default_shortcut: Some("Ctrl+Shift+C"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "pasteclip", label: "Paste", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+V"), secondary_shortcut: None },
    CommandSpec {
        id: "selectall",
        label: "Select All",
        category: CommandCategory::Edit,
        default_shortcut: Some("Ctrl+A"),
        secondary_shortcut: Some("AI_SELALL"),
    },
    // Draw
    CommandSpec { id: "line", label: "Line", category: CommandCategory::Draw, default_shortcut: Some("L"), secondary_shortcut: None },
    CommandSpec { id: "pline", label: "Polyline", category: CommandCategory::Draw, default_shortcut: Some("PL"), secondary_shortcut: None },
    CommandSpec { id: "circle", label: "Circle", category: CommandCategory::Draw, default_shortcut: Some("C"), secondary_shortcut: None },
    CommandSpec { id: "arc", label: "Arc", category: CommandCategory::Draw, default_shortcut: Some("A"), secondary_shortcut: None },
    CommandSpec { id: "rectang", label: "Rectangle", category: CommandCategory::Draw, default_shortcut: Some("REC"), secondary_shortcut: None },
    CommandSpec { id: "ellipse", label: "Ellipse", category: CommandCategory::Draw, default_shortcut: Some("EL"), secondary_shortcut: None },
    CommandSpec { id: "hatch", label: "Hatch", category: CommandCategory::Draw, default_shortcut: Some("H"), secondary_shortcut: None },
    CommandSpec { id: "mtext", label: "Multiline Text", category: CommandCategory::Draw, default_shortcut: Some("T"), secondary_shortcut: None },
    // Modify
    CommandSpec { id: "erase", label: "Erase", category: CommandCategory::Modify, default_shortcut: Some("E"), secondary_shortcut: Some("Delete") },
    CommandSpec { id: "move", label: "Move", category: CommandCategory::Modify, default_shortcut: Some("M"), secondary_shortcut: None },
    CommandSpec { id: "copy", label: "Copy", category: CommandCategory::Modify, default_shortcut: Some("CO"), secondary_shortcut: None },
    CommandSpec { id: "rotate", label: "Rotate", category: CommandCategory::Modify, default_shortcut: Some("RO"), secondary_shortcut: None },
    CommandSpec { id: "scale", label: "Scale", category: CommandCategory::Modify, default_shortcut: Some("SC"), secondary_shortcut: None },
    CommandSpec { id: "mirror", label: "Mirror", category: CommandCategory::Modify, default_shortcut: Some("MI"), secondary_shortcut: None },
    CommandSpec { id: "offset", label: "Offset", category: CommandCategory::Modify, default_shortcut: Some("O"), secondary_shortcut: None },
    CommandSpec { id: "trim", label: "Trim", category: CommandCategory::Modify, default_shortcut: Some("TR"), secondary_shortcut: None },
    CommandSpec { id: "extend", label: "Extend", category: CommandCategory::Modify, default_shortcut: Some("EX"), secondary_shortcut: None },
    CommandSpec { id: "fillet", label: "Fillet", category: CommandCategory::Modify, default_shortcut: Some("F"), secondary_shortcut: None },
    CommandSpec { id: "chamfer", label: "Chamfer", category: CommandCategory::Modify, default_shortcut: Some("CHA"), secondary_shortcut: None },
    CommandSpec { id: "explode", label: "Explode", category: CommandCategory::Modify, default_shortcut: Some("X"), secondary_shortcut: None },
    // Layers
    CommandSpec {
        id: "layer",
        label: "Layer Properties…",
        category: CommandCategory::Layers,
        default_shortcut: Some("LA"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "layer.new", label: "New Layer", category: CommandCategory::Layers, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "layer.current",
        label: "Set Current Layer",
        category: CommandCategory::Layers,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // View
    CommandSpec { id: "zoom.window", label: "Zoom Window", category: CommandCategory::View, default_shortcut: Some("Z"), secondary_shortcut: None },
    CommandSpec {
        id: "zoom.extents",
        label: "Zoom Extents",
        category: CommandCategory::View,
        default_shortcut: Some("Z E"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "zoom.previous",
        label: "Zoom Previous",
        category: CommandCategory::View,
        default_shortcut: Some("Z P"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "pan", label: "Pan Realtime", category: CommandCategory::View, default_shortcut: Some("P"), secondary_shortcut: None },
    CommandSpec { id: "regen", label: "Regen", category: CommandCategory::View, default_shortcut: Some("RE"), secondary_shortcut: None },
    CommandSpec { id: "redraw", label: "Redraw", category: CommandCategory::View, default_shortcut: Some("R"), secondary_shortcut: None },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().label, cmd.label);
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Draw).is_empty());
        assert!(!commands_by_category(CommandCategory::Modify).is_empty());
        assert!(!commands_by_category(CommandCategory::Layers).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }

    #[test]
    fn test_registry_ids_exist_in_engine() {
        for cmd in COMMAND_REGISTRY {
            assert!(cadcraft_engine::find_command(cmd.id).is_some(), "registry id `{}` not in the engine command catalog", cmd.id);
        }
    }
}
