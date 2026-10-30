# Upstream provenance

Derived from gpui 0.2.2, distributed by the Zed/GPUI contributors under Apache-2.0. The original LICENSE-APACHE is preserved. This package is a Rusty GPUI compatibility distribution under a distinct name, not an official GPUI release.

The runtime is based on upstream GPUI 0.2.2. Its manifest routes the http_client dependency to rusty-gpui-http, allowing Rusty GPUI to fix the obsolete xattr dependency without asking applications to patch the runtime. The grid-repeat helper explicitly types its zero/fraction literals as f32 for current Rust compatibility; this preserves its existing behavior. Use the facade reexport or declare the dependency alias gpui with package = "rusty-gpui-runtime". Do not mix these GPUI types with the registry gpui package in the same app.

The macOS key-status callback releases the window-state mutex before calling AppKit's `resignKeyWindow`. AppKit synchronously invokes the matching resignation callback, which locks that same state. Holding the guard across this call caused a reproduced native activation deadlock.

Keep manifest changes narrow and document upstream upgrades. Release version follows the Rusty GPUI workspace; upstream version remains recorded separately.
