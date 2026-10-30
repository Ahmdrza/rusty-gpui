# Rusty GPUI

Main crate: **`rusty-gpui`**. Source: [Ahmdrza/rusty-gpui](https://github.com/Ahmdrza/rusty-gpui).

A configurable native Rust design system for GPUI. Shared foundations for actions, fields, selection, navigation, feedback, overlays and data, with an interactive gallery for development.

**0.4.0 is a release candidate prepared for publication. Nothing has been uploaded to crates.io yet.**

The current renderer does not expose semantic controls to platform screen readers. Applications requiring screen-reader support should wait for runtime accessibility integration. See [production readiness gates](docs/production-readiness.md) for the remaining platform and accessibility work.

## Packages

| Package | Role | Published? |
|---|---|---|
| `rusty-gpui` | Public entry point: theme integration, components, assets, typography and layout | Planned |
| `rusty-gpui-runtime` | GPUI 0.2.2 compatibility runtime, used internally | Planned |
| `rusty-gpui-http` | Narrow HTTP/archive compatibility dependency | Planned |
| `rusty-gpui-theme` | Dependency-free semantic colors and design tokens | Planned |
| `rusty-gpui-components` | Maintained native component fork with Rusty GPUI foundations and fixes | Planned |
| `rusty-gpui-gallery` | Interactive showcase and development workbench | Never; `publish = false` |

Applications depend on `rusty-gpui`. The gallery is not part of the library packages. No consumer-side Cargo patches are required.

## Use locally

```toml
[dependencies]
rusty-gpui = { path = "../rusty-gpui/crates/rusty-gpui" }
# Only needed if your code imports gpui directly; it is also reexported by Rusty GPUI.
gpui = { package = "rusty-gpui-runtime", version = "=0.4.0", path = "../rusty-gpui/crates/rusty-gpui-runtime", default-features = false }
```

```rust,ignore
use rusty_gpui::prelude::*;
use rusty_gpui::gpui::*;

// Application::new().with_assets(rusty_gpui::Assets).run(|cx| { ... });
// Initialize Rusty GPUI once, wrap the view in Root and render its overlay layers.
rusty_gpui::init(cx);
```

The [minimal app](crates/rusty-gpui/examples/minimal.rs) demonstrates the complete setup:

```sh
cargo run --locked -p rusty-gpui --example minimal
```

For shorter setup, use `rusty_gpui::run("My app", build_view)`. It handles assets, fonts, theme, the window, Root and overlay layers. Copy a complete [hello app](crates/rusty-gpui/examples/hello.rs), [form](crates/rusty-gpui/examples/form.rs), or [dialog/drawer example](crates/rusty-gpui/examples/overlays.rs):

```sh
cargo run -p rusty-gpui --example hello
cargo run -p rusty-gpui --example form
cargo run -p rusty-gpui --example overlays
```

Coming from npm? See [Rusty GPUI for Node.js developers](docs/coming-from-node.md) for Cargo equivalents, local app setup, and how GitHub source, crate packaging and examples fit together. The repository keeps its examples; consuming applications build the library without building those example executables.

Once the planned packages are published, the dependency becomes `rusty-gpui = "0.4"`. This registry command will not work until publication. Git dependencies will also work after the repository is pushed, using `package = "rusty-gpui"` from this workspace.

## Gallery

On macOS install Rust and full Xcode, open Xcode once, accept its license and select its Command Line Tools location. GPUI needs the Metal compiler; Command Line Tools alone may be insufficient. Then run:

```sh
bash scripts/run-macos.sh
```

If Metal is a separate Xcode download: `xcodebuild -downloadComponent MetalToolchain`. The first build downloads dependencies; later builds reuse Cargo's cache. Linux requires GPUI's Vulkan, X11/Wayland and font development libraries; see the native CI workflow for the package list.

The gallery has working editing, search, menus, selections, dialogs, drawers, notifications and a virtualized table. Its inspector changes color seeds, light/dark appearance, density, typography, spacing and radius. Settings save to `~/Library/Application Support/Rusty GPUI/theme.json` on macOS; RUSTY_GPUI_CONFIG_DIR overrides the directory.

## Configure

```rust,ignore
use rusty_gpui::theme::{Color, Density, Mode, ThemeConfig};
let config = ThemeConfig {
    primary: Color::hex("#247A58")?,
    mode: Mode::Dark,
    density: Density::Comfortable,
    font_family: "Inter".into(),
    font_size: 14.0,
    spacing_unit: 4.0,
    radius: 5.0,
    ..Default::default()
};
rusty_gpui::apply_theme(config, Some(window), cx);
```

Use the `actions`, `fields`, `typography` and `layout` modules or their prelude. The lower-level catalog is reexported as `rusty_gpui::components`; the matching runtime as `rusty_gpui::gpui`; tokens as `rusty_gpui::theme`. The theme is app-wide. Use the matching Rusty GPUI runtime types, not registry GPUI types. Do not combine the fork's Root/theme with upstream GPUI Component in one component tree: they are distinct implementations.

## Development and release

- [Architecture and extension rules](docs/architecture.md)
- [Component catalog](docs/components.md) and [field composition](docs/fields.md)
- [Contributing](CONTRIBUTING.md)
- [Release guide](RELEASING.md), including repository setup, package checks and publish order
- [Changelog](CHANGELOG.md) and [0.4.0 changes](docs/changes-0.4.0.md)
- [Native dependency audit](docs/dependency-audit.md)

Release preparation packages the five libraries, stages their registry artifacts locally and compiles a separate consumer with a version-only dependency. This checks the distribution boundary without publishing anything. It needs Python 3.11+ and cached Cargo dependencies; see RELEASING.md.

The runtime and HTTP compatibility packages eliminate the obsolete xattr dependency by disabling unused archive-xattr support. There are no Cargo patch tables in this workspace or in consumer manifests. The runtime retains upstream GPUI 0.2.2 with explicit grid-sizing float types for current Rust compatibility; provenance and the narrow manifest changes are documented in each compatibility crate.

## Status and licensing

Native macOS and Linux are the release targets. Linux compilation was checked during the source handoff. Core macOS editing, keyboard, search, overlay and scroll interactions have been exercised locally; see [native review](docs/local-macos-review.md) for exact coverage. Assistive technology and platform performance remain unverified. Specialized widgets have not all received a dedicated Rusty GPUI presentation migration. Frame samples in the gallery measure render callbacks, not GPU execution time.

Rusty GPUI facade/theme code is MIT licensed. Rusty GPUI Components retains Apache-2.0 and upstream provenance in [UPSTREAM.md](crates/rusty-gpui-components/UPSTREAM.md). Inter is SIL OFL; icon notices are ISC/MIT with upstream distribution notices. Each published package includes its applicable licenses.
