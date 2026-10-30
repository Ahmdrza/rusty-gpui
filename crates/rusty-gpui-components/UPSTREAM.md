# Upstream provenance

Rusty GPUI Components is a maintained fork of GPUI Component 0.5.1, originally by Longbridge and its contributors: https://github.com/longbridge/gpui-component. It is published under a distinct package name, rusty-gpui-components, so consumers receive Rusty GPUI changes without Cargo patch entries. It is not an official Longbridge release.

The original Apache-2.0 license is preserved in LICENSE-APACHE. Rusty GPUI modifications are also distributed under Apache-2.0 in this crate. Upstream macros remain dependencies under their original names. Cargo.toml.orig and the original lockfile are historical references and are not published.

Rusty GPUI modifications include shared control/field presentation, semantic styling, disabled focus/caret/cursor handling, button foreground and icon-only fixes, contained table scrolling, zero default sheet offset, dropdown trigger-width alignment, validation fields and keyboard-focusable switches. See the repository architecture and change notes for ownership and behavioral details.

Deprecated std::usize imports in the highlighter were removed so existing usize::MIN/MAX references use primitive associated constants on current Rust.

Root constrains Tab and Shift-Tab traversal to the top active dialog or sheet. Upstream tab groups order focus but do not prevent it from moving into the obscured page.

Password reveal uses the native button click path, supports Tab and keyboard activation, and toggles masking instead of requiring a held mouse button. Its tooltip describes the current Show/Hide action; disabled inputs cannot activate it.

Table resize dividers retain their thin appearance with a 6px hit area, reducing accidental header reordering when trying to resize a column.

Future upstream updates must preserve these changes, update this provenance record and be reviewed against the gallery and isolated package checks. The fork's release version follows Rusty GPUI; upstream 0.5.1 is recorded separately and is not a compatibility promise for future Rusty GPUI versions.
