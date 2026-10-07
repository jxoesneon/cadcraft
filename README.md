# CADCraft Studio

An open-source, sovereign computer-aided drafting and 2D/3D design application built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![CADCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode CAD interface featuring a CLI prompt HUD, orthogonal snap indicator, and precision crosshairs.
- **`crates/engine`**: Infinite-precision geometric solver, DXF/DWG format parsers, and GPU-accelerated vector drafting canvas.

## Legal & Compliance Notice

CADCraft is an independent open-source drafting application. It is not affiliated with Autodesk Inc. Autodesk, AutoCAD, and DWG are trademarks of Autodesk Inc. Command-line interface interactions and geometric drafting commands are uncopyrightable methods of operation (17 U.S.C. § 102(b)).

## License

Dual-licensed under MIT OR Apache-2.0.
