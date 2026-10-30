# Copyable applications

Run from the repository root:

| Command | Demonstrates |
| --- | --- |
| `cargo run -p rusty-gpui --example hello` | Text, button and notification with one-line app setup |
| `cargo run -p rusty-gpui --example form` | Input state, validation, checkbox, switch and Save callback |
| `cargo run -p rusty-gpui --example overlays` | Dialog and drawer with automatic overlay layers |
| `cargo run -p rusty-gpui --example minimal` | Explicit assets, initialization, window, Root and overlay setup |

The first three files can be copied directly into your application's `src/main.rs` with a single rusty-gpui dependency. The minimal example uses a direct gpui import; add `use rusty_gpui::gpui;` when copying it into an app that only depends on rusty-gpui.

`rusty_gpui::run` owns the window's Root and overlay layers. Views using that helper render their content and call WindowExt methods to open overlays. The minimal example shows the manual alternative for custom application/window options.

Examples remain in GitHub and the published source archive; Cargo does not build dependency examples in a normal consuming application. The separate gallery is not published.
