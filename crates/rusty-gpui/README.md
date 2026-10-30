# Rusty GPUI

A configurable native Rust design system for GPUI. Shared component foundations, theme generation, bundled Inter and SVG icons, typography, layout and native controls.

```toml
[dependencies]
rusty-gpui = "0.4"
# If you import gpui directly rather than through rusty_gpui::gpui:
gpui = { package = "rusty-gpui-runtime", version = "=0.4.0", default-features = false }
```

For local library development, use a path to this crate. No consumer patch entries or gallery dependency are required.

## First application

Create a binary crate with `cargo new my-app`, add the dependency above, and replace `src/main.rs` with:

```rust,no_run
use rusty_gpui::gpui::{prelude::*, *};
use rusty_gpui::prelude::TextStyle;
use rusty_gpui::prelude::*;

struct Hello;

impl Render for Hello {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        stack(cx)
            .p_6()
            .child(text("Hello, Rusty GPUI", TextStyle::Heading, cx))
            .child(
                button("hello", "Say hello", ActionStyle::Primary, cx)
                    .on_click(|_, window, cx| window.push_notification("Hello!", cx)),
            )
    }
}

fn main() {
    rusty_gpui::run("Hello", |_, cx| cx.new(|_| Hello));
}
```

Run `cargo run`. The `rusty_gpui::run` helper supplies assets, fonts, the default theme, a window, Root, overlay layers and shutdown when the last window closes. Your view contains just its UI. Stateful controls use GPUI entities and callbacks.

From the source workspace, try these complete examples:

```sh
cargo run -p rusty-gpui --example hello
cargo run -p rusty-gpui --example form
cargo run -p rusty-gpui --example overlays
```

`hello` demonstrates text, a button and a notification. `form` adds an editable input, validation, checkbox, switch and a Save callback that reads the current values. `overlays` demonstrates dialogs and drawers with automatic overlay layers. `minimal` retains the explicit GPUI setup for applications needing custom assets, window options or lifecycle behavior. Each example can be copied into your own `src/main.rs`; `minimal` additionally needs `use rusty_gpui::gpui;` for its direct GPUI imports. The gallery is a separate, unpublished development application.

The GitHub repository contains the libraries, gallery, examples, tests and development tools. Cargo publishes selected library files; see each manifest's `include` list. Examples are included as source documentation, but `cargo build` in a consuming app does not build or launch a dependency's examples or gallery. This works like keeping an examples folder in an npm package: applications choose which library APIs they use.

The public API reexports GPUI as `rusty_gpui::gpui`, the token engine as `theme`, and the maintained native component catalog as `components`. Choose either the facade's reexports or the matching rusty-gpui-runtime dependency alias; do not mix the fork with an upstream GPUI Component Root/theme.

The theme model is app-wide. Component helpers retain native editing, focus and popup behavior. macOS requires full Xcode/Metal; Linux requires GPUI's graphics/windowing development libraries. Windows is not a release-tested target.

Rusty GPUI facade code is MIT licensed. Inter is SIL OFL; icons include ISC/MIT notices and upstream distribution notices. The native component fork is Apache-2.0. All notices are included in packages.

The Rusty GPUI runtime is a compatibility distribution of GPUI 0.2.2. It routes HTTP support through a narrow compatibility crate that disables unused archive xattr support, avoiding obsolete libc references on Linux. These packages use distinct names and preserve Apache-2.0 provenance. Do not add registry gpui 0.2.2 alongside the Rusty GPUI runtime; their types have different crate identities.

## Customization and support

Use `rusty_gpui::apply_theme(config, Some(window), cx)` with a `theme::ThemeConfig` to change appearance, accent, density, font, spacing and radius together. `RustyField` composes labels, required markers, help and errors around native fields. `actions`, `fields`, `typography` and `layout` provide the main application helpers; `components` exposes the full catalog.

Current stable Rust is the supported baseline. First native builds can take several minutes; keep Cargo's target cache. On macOS, select full Xcode and ensure `xcrun --find metal` succeeds. Linux requires a graphical session, GPU drivers and the native packages listed in [CONTRIBUTING.md](https://github.com/Ahmdrza/rusty-gpui/blob/main/CONTRIBUTING.md). Windows is experimental and unverified.

This is an early pre-1.0 release. Breaking public API changes increment the minor version; compatible fixes increment the patch version. Only one app-wide theme is supported. The current renderer does not expose semantic controls to platform screen readers; applications requiring that support must wait for runtime accessibility integration. Windows and specialized upstream widgets remain unverified. Use the gallery and starter to evaluate the controls your application needs before deploying them.
