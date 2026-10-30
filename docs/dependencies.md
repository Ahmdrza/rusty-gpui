# Dependencies

Applications normally install only `rusty-gpui`. Cargo brings in its supporting libraries automatically.

| Package | Role |
| --- | --- |
| `rusty-gpui` | Public helpers, assets and theme integration. |
| `rusty-gpui-components` | Native controls, derived from GPUI Component 0.5.1. |
| `rusty-gpui-theme` | Renderer-independent colors and design tokens. |
| `rusty-gpui-runtime` | Windowing, rendering, input and application state, derived from GPUI 0.2.2. |
| `rusty-gpui-http` | GPUI networking support, including remote image and download helpers. |

The runtime and HTTP distributions allow compatibility fixes without requiring application-level Cargo patches. The HTTP package disables unused archive extended-attribute support to avoid an obsolete Linux dependency. The runtime also contains a macOS activation deadlock fix. The component fork carries shared styling and interaction fixes.

These are maintained forks, not official upstream releases. Preserve their Apache-2.0 licenses and `UPSTREAM.md` provenance when updating. Keeping the forks synchronized with upstream fixes is an ongoing maintenance responsibility.

Use `rusty_gpui::gpui` for the matching runtime types. Do not mix them with the separate registry `gpui` package or upstream GPUI Component's Root/theme globals.

Inspect the current dependency graph with `cargo tree` or `python3 scripts/dependency-report.py`. Platform features affect the graph: macOS uses native Metal and font-kit, while Linux enables X11 and Wayland.
