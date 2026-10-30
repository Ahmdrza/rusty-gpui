# Rusty GPUI

A configurable native Rust component library for GPUI, with themes, bundled fonts and icons, and an interactive development gallery.

[crates.io](https://crates.io/crates/rusty-gpui) · [API documentation](https://docs.rs/rusty-gpui) · [Examples](crates/rusty-gpui/examples)

## Install

```sh
cargo add rusty-gpui
```

Create an app with `cargo new my-app`, add the dependency, and copy the complete [hello example](crates/rusty-gpui/examples/hello.rs) into `src/main.rs`. Then run `cargo run`.

`rusty_gpui::run("My app", build_view)` sets up the window, bundled assets, fonts, default theme and overlay layers. Import the matching GPUI APIs through `rusty_gpui::gpui`; applications normally need only the `rusty-gpui` dependency.

## Examples and gallery

From this repository:

```sh
cargo run -p rusty-gpui --example hello
cargo run -p rusty-gpui --example form
cargo run -p rusty-gpui --example overlays
cargo run -p rusty-gpui-gallery
```

The examples cover text and buttons, editable forms with validation, checkboxes, switches, dialogs, drawers and notifications. The [minimal example](crates/rusty-gpui/examples/minimal.rs) demonstrates explicit setup for custom window options and application lifecycle.

The gallery includes component demos, a theme inspector and a virtualized table. Examples and the gallery stay in this repository; consuming apps do not build them automatically.

macOS requires full Xcode and the Metal compiler. Linux requires graphics, windowing and font libraries. See [native prerequisites](CONTRIBUTING.md#native-prerequisites).

## Customize

Configure appearance, colors, density, typography, spacing and radius using `rusty_gpui::theme::ThemeConfig`, then apply it with `rusty_gpui::apply_theme(config, Some(window), cx)`. Themes apply across the application.

Use the helpers in `actions`, `fields`, `typography` and `layout`, or their prelude. The full component catalog is available through `rusty_gpui::components`.

- [Component catalog](docs/components.md)
- [Field composition](docs/fields.md)
- [Architecture](docs/architecture.md)
- [Packages and dependencies](docs/dependencies.md)

## Support

This is an early pre-1.0 library. macOS native interactions and macOS/Linux CI builds have been checked. Windows is experimental. Custom controls currently lack semantic screen-reader support. Review [support and limitations](docs/support.md) before adoption.

## Contribute

See [Contributing](CONTRIBUTING.md), the [release guide](RELEASING.md) and the [changelog](CHANGELOG.md).

## License

The facade and theme are MIT licensed. The runtime, HTTP and component forks retain Apache-2.0 and their `UPSTREAM.md` attribution. Bundled Inter fonts use SIL OFL; icons include ISC/MIT notices. Each crate includes its applicable licenses.
