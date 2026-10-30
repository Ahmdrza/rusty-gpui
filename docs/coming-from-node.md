# Rusty GPUI for Node.js developers

The GitHub repository and crates.io package use the same library source. GitHub holds the whole project; crates.io receives selected files for each library. Cargo reads each crate's `Cargo.toml` to decide what to package. You do not maintain a separate implementation for crates.io. The release workflow is the same idea as npm: edit one checkout, push it to GitHub, and publish a package from that checkout. `git push` and `cargo publish` are two commands, just as `git push` and `npm publish` are.

| Node.js / npm | Rust / Cargo |
| --- | --- |
| `package.json` | `Cargo.toml`: metadata, dependencies and build targets |
| `package-lock.json` | `Cargo.lock`: resolved dependency versions |
| `npm install some-package` | `cargo add some-crate` adds the dependency; builds fetch it automatically |
| `npm run dev` / start script | `cargo run`: build and launch the app; no automatic hot reload |
| `npm test` | `cargo test` |
| `npm ls` | `cargo tree` |
| `npm publish` | `cargo publish`: uploads one crate |

Cargo caches downloaded dependencies outside the project and keeps build output in `target/`. Keep the target cache locally; keep it out of Git. Application code is normally in `src/main.rs`; library code is in `src/lib.rs` and its modules.

The dependency is named `rusty-gpui` in Cargo.toml, while Rust imports use `rusty_gpui`. That hyphen-to-underscore conversion is normal for Rust crates.

Cargo compiles your application and its Rust dependencies into a native executable. Your application's users run that executable; they do not need to install Cargo or Rusty GPUI themselves. Distributing a polished desktop app still needs the usual platform packaging/signing work.

## Try the examples

From the Rusty GPUI repository:

```sh
cargo run -p rusty-gpui --example hello
cargo run -p rusty-gpui --example form
cargo run -p rusty-gpui --example overlays
```

`hello` is the smallest app. `form` has an input, validation, checkbox, switch and Save notification. `overlays` opens a dialog and a drawer. `minimal` shows explicit GPUI setup when you need more control.

The `rusty_gpui::run("Window title", build_view)` helper sets up assets, fonts, the default theme, window, Root, overlay layers and shutdown. Each example is a complete file that you can copy into your application's `src/main.rs`.

## Start your own app today

Rusty GPUI 0.4.0 has not been published yet. Create your app **beside** the Rusty GPUI checkout:

```text
projects/
  rusty-gpui/
  my-app/
```

From `projects/`:

```sh
cargo new my-app
cd my-app
```

Add this dependency to `my-app/Cargo.toml`:

```toml
[dependencies]
rusty-gpui = { path = "../rusty-gpui/crates/rusty-gpui" }
```

Copy `rusty-gpui/crates/rusty-gpui/examples/hello.rs` into `my-app/src/main.rs`, then run `cargo run`. Use the form and overlays examples as you add components. They import the matching runtime through `rusty_gpui::gpui`, so your app needs just the facade dependency.

After registry publication, the dependency becomes `rusty-gpui = "0.4"`; `cargo add rusty-gpui` will then work. Platform prerequisites still apply: full Xcode/Metal on macOS and the graphics/windowing libraries in CONTRIBUTING.md on Linux.

## Keep examples in GitHub

The repository is a Cargo workspace containing five libraries and the development gallery. The gallery has `publish = false` and is absent from library archives. Each library has an explicit `include` list. The facade includes its library source, required fonts/icons, README, license and small examples.

Examples are useful documentation and do not run in a consuming app. A normal application build compiles the library APIs it depends on; it does not build a dependency's example executables. `cargo run --example form` explicitly selects an example in the workspace. Keep examples in the repository so contributors and users can learn the API. No example-removal step is needed before publication.

Use `cargo package --list -p rusty-gpui` to inspect the facade's package contents. Follow RELEASING.md for the full five-crate verification and dependency order. Nothing in the example commands publishes a package.

This simpler setup does not change the platform/support limits in [production readiness](production-readiness.md).
