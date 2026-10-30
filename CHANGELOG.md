# Changelog

## 0.4.0 — Unreleased

- Add `rusty_gpui::run` for automatic single-window setup and overlay layers, with copyable hello, form and overlay apps and a Cargo guide for Node.js developers.

- Rename the project to Rusty GPUI, with rusty-gpui for the main crate and GitHub repository; align support crates, imports, launchers and CI. Preserve legacy type aliases and gallery preference fallback.

- Prepare five libraries for independent distribution: rusty-gpui-http, rusty-gpui-runtime, rusty-gpui-theme, rusty-gpui-components and rusty-gpui.
- Replace the workspace gpui-component patch with a distinct versioned native component fork; remove consumer patch requirements.
- Remove the transitive xattr workaround using narrowly documented HTTP/runtime compatibility distributions; no patches remain.
- Keep the gallery unpublished and separate from library archives.
- Include package-level documentation, licenses, native assets and a standalone example; expose GPUI through the facade.
- Add isolated registry-consumer checks, native CI, repository metadata tooling and a dependency-ordered release guide.
- Add a complete compile-checked application quickstart and build the packaged starter with only a versioned facade dependency.
- Set source metadata to https://github.com/Ahmdrza/rusty-gpui and correct the gallery's remaining Form header label.
- Fix Rust float fallback warnings in runtime grid sizing and deprecated highlighter imports; preserve fork provenance.
- Keep Tab and Shift-Tab focus inside the active dialog or drawer instead of moving into the dimmed page.
- Fix a macOS activation deadlock by releasing window state before AppKit's synchronous resignation callback.
- Make password reveal keyboard accessible with a native Show/Hide toggle; widen table resize hit areas.
- Fix the gallery header search's first-character layout and retain column identity when table headers are moved.
- Adapt gallery navigation and theme controls to compact windows, with reproducible resize and dynamic picker-disable demonstrations.
- Run packaged-consumer builds for macOS and Linux on GitHub pushes and pull requests; document the runtime accessibility gap as a production blocker.

## 0.3.0

Reusable field validation/help, consistent disabled focus, keyboard-focusable switches, bundled icons, standalone starter and dependency audit. See docs/changes-0.3.0.md.

## 0.2.0

Shared foundations across the main component families; modular facade. See docs/changes-0.2.0.md.

## 0.1.x

Initial native gallery, configurable tokens and interaction fixes. Historical release notes are in docs.
