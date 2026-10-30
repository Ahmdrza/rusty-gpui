# Rusty GPUI 0.3.0

- Added RustyField, composing the existing label layout with a description, required marker and error message. Input, Select, DatePicker and NumberInput share invalid presentation; disabled state takes precedence over the error border.
- Switches now participate in keyboard focus and use GPUI's native click dispatch for mouse and keyboard activation.
- Shared focus policy removes disabled controls from tab order and drops existing focus. Disabled selects guard actions; disabling a select/date closes its popup.
- Rusty GPUI now bundles its icon Assets provider. Removed the separate asset crate and selected GPUI backend features by target, preserving macOS font rendering and Linux X11/Wayland.
- Apple Silicon normal/build dependency graph reduced from 512 to 493 unique package versions. Added a reproducible dependency report; this is not a build-time claim.
- Added a minimal native starter app independent of gallery state and settings. Existing visual direction and configurable foundations remain.

Migration: replace gpui_component_assets::Assets with rusty_gpui::Assets and remove the old asset dependency. Set default-features = false on direct GPUI dependencies to preserve platform selection. Existing component APIs remain available.

Validation: workspace and starter-example compile checks, formatting check and icon-path integrity check. No GUI automation or test suites run, following the requested quick handoff workflow. Mac runtime verification remains with the user.
