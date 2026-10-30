# Rusty GPUI 0.4.0 — Unreleased

Prepared for GitHub and crates.io without publishing anything. Renamed to Rusty GPUI; the public crate and planned GitHub repository are both rusty-gpui.

- Five distributable libraries: rusty-gpui-http, rusty-gpui-runtime, rusty-gpui-theme, rusty-gpui-components and rusty-gpui. Gallery explicitly unpublished.
- Versioned path dependencies normalize to registry requirements; the native component fork replaces consumer-side gpui-component patches.
- Applicable licenses, notices, upstream provenance, per-crate README files, fonts, icons and starter example included in the library packages.
- GPUI reexported through the facade; applications can depend only on rusty-gpui when using its reexports.
- Added isolated registry-consumer validation, focused macOS/Linux CI, manual candidate-package workflow and release/versioning guide.
- Repository metadata is set to https://github.com/Ahmdrza/rusty-gpui. Names were checked for existing crates.io entries during preparation; nothing reserves them yet.

The clean-consumer check found xattr was a normal transitive dependency, not development-only as initially assumed. The runtime/HTTP compatibility distributions eliminate it by disabling unused archive xattr support. No patch tables remain. Existing 0.3.0 visuals and component behaviors remain.

Validation includes native macOS gallery/starter interaction checks, focused unit checks and release packaging. See local-macos-review.md and release-validation-0.4.0.json for current results and limits.

Release hardening adds a complete compile-checked README quickstart, builds the packaged starter as an independent consumer, and fixes current Rust warnings in grid sizing and highlighter imports.
