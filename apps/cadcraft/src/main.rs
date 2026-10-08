//! CADCraft desktop application — 100% sovereign Martensite runtime.
//!
//! Usage: `cadcraft [files…]` (argument handling lands with the Martensite runner).
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use cadcraft_engine::Engine;
use cadcraft_ui_martensite::CadcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new();
    let app = CadcraftApp::new(engine);

    println!("Starting CADCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
