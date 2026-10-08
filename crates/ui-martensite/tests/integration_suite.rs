//! Comprehensive integration test suite for CADCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! scrubby inputs, layer state, status-bar aids, and canvas coordinates.

use cadcraft_engine::{Engine, Tool};
use cadcraft_ui_martensite::{
    CadcraftApp,
    command_reg::find_command,
    menus::generate_main_menu,
    theme::CraftTheme,
    widgets::{DockPanelGroup, LayerItemDef, LayerTreeWidget, OptionsBarWidget, StatusBarWidget},
};

fn layer(id: u64, name: &str) -> LayerItemDef {
    LayerItemDef {
        id,
        name: name.to_string(),
        visible: true,
        frozen: false,
        locked: false,
        color: [255, 255, 255],
        is_current: false,
        children: vec![],
    }
}

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = CadcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Select);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.snap_mode);
    assert!(app.grid_visible);
    assert!(!app.ortho_mode);

    // 2. Keystroke Workflow: switch to Line, zoom in, hold Space to pan
    let new_tool = app.keyboard.on_key_down("l", app.active_tool);
    assert_eq!(new_tool, Some(Tool::Line));
    app.set_tool(Tool::Line);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Pan tool
    let pan_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(pan_tool, Some(Tool::Pan));
    app.set_tool(Tool::Pan);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores Line
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::Line));
    app.set_tool(Tool::Line);

    // 3. Command-line HUD: type a command like at the AutoCAD prompt
    app.prompt.current_input = "circle".to_string();
    assert_eq!(app.prompt.submit_command(), Some("CIRCLE".to_string()));
    assert_eq!(app.prompt.history.len(), 1);

    // 4. Options Bar Interaction for the running command
    let mut options = OptionsBarWidget::new();
    options.set_tool(Tool::Circle);
    options.radius.on_pointer_down(0.0);
    options.radius.on_pointer_move(20.0, false, false);
    options.radius.on_pointer_up();
    assert_eq!(options.radius.value, 30.0); // 10 + 20

    // 5. Layer List & Hierarchy Updates
    let mut layers = LayerTreeWidget::new();
    layers.layers.push(layer(1, "0"));
    layers.layers.push(layer(2, "Walls"));
    layers.select_layer(2);
    assert_eq!(layers.selected_layer_id, Some(2));
    layers.set_current(2);
    assert_eq!(layers.current_layer_id, Some(2));
    layers.toggle_frozen(2);
    assert!(layers.layers[1].frozen);
    layers.toggle_visibility(2);
    assert!(!layers.layers[1].visible);

    // 6. Status Bar Drafting Aids
    let mut status = StatusBarWidget::new();
    assert_eq!(status.toggle("ortho"), Some(true));
    assert!(status.ortho);
    assert_eq!(status.toggle("osnap"), Some(false));
    assert!(!status.osnap);

    // 7. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Properties", "Layers", "Tool Palettes"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 8. Menu Generation Consistency
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {}", cmd_id);
            }
        }
    }

    // 9. Theme Color Space Consistency
    let theme = CraftTheme::dark_neutral();
    let obsidian = CraftTheme::studio_obsidian();
    assert_ne!(theme.surface_app_bg, obsidian.surface_app_bg);
}
