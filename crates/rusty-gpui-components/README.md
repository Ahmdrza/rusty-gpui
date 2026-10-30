# Rusty GPUI Components

Native GPUI controls and shared presentation primitives used by the Rusty GPUI design system. Most applications should depend on **rusty-gpui**, which supplies theme generation, bundled assets, curated adapters and layout helpers.

This is a maintained Apache-2.0 fork of GPUI Component 0.5.1, published under its own name. It is not an official upstream release. See [UPSTREAM.md](UPSTREAM.md) for provenance and the preserved [license](LICENSE-APACHE).

Direct users import `rusty_gpui_components`. Rusty GPUI's facade exposes the same catalog as `rusty_gpui::components`. Keep one implementation in an application; upstream GPUI Component and this fork have distinct Root/theme globals and are not interchangeable.

Supported release targets are macOS and Linux. Platform dependencies enable macOS font rendering and Linux X11/Wayland. GPUI's native prerequisites still apply. The complete component catalog includes specialized widgets that have not all received dedicated Rusty GPUI visual review.
