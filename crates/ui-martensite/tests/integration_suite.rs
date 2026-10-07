//! CAD integration tests.

use cadcraft_ui_martensite::CadcraftApp;

#[test]
fn test_cad_workflow() {
    let mut app = CadcraftApp::new();
    app.prompt.current_input = "circle".to_string();
    assert_eq!(app.prompt.submit_command(), Some("CIRCLE".to_string()));
    assert!(!app.ortho_mode);
    app.toggle_ortho();
    assert!(app.ortho_mode);
}
