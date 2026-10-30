# Contributing

Use current stable Rust and native GPUI prerequisites. The workspace lockfile is committed for reproducible gallery and CI builds; consuming libraries resolve their own graphs. Python 3.11+ is needed for release tooling.

Keep app-specific behavior in apps/gallery or a separate application. Changes to controls belong in the appropriate shared foundation and behavioral component, not only in a gallery wrapper. See docs/architecture.md. Keep public API additions small and document compatibility changes in CHANGELOG.md.

## Native prerequisites

On macOS, install full Xcode, open it and complete its first-launch setup. Select it in Xcode Settings → Locations → Command Line Tools. Verify `xcode-select -p` points inside Xcode.app and `xcrun --find metal` succeeds. If Metal is a separate download for that Xcode version, run `xcodebuild -downloadComponent MetalToolchain`.

On Debian/Ubuntu, the CI dependency set is:

```sh
sudo apt-get update
sudo apt-get install -y clang cmake pkg-config libasound2-dev libssl-dev libudev-dev libfontconfig1-dev libfreetype6-dev libglib2.0-dev libwayland-dev libx11-dev libx11-xcb-dev libxcb1-dev libxcb-shm0-dev libxcb-xinput-dev libxcb-xkb-dev libxkbcommon-dev libxkbcommon-x11-dev libvulkan-dev
```

Running the application also requires an X11 or Wayland graphical session and a working Vulkan driver. Equivalent package names vary by distribution. Windows is experimental and is not part of release validation.

## Checks

```sh
cargo fmt -p rusty-gpui-theme -p rusty-gpui -p rusty-gpui-gallery -p rusty-gpui-settings --check
cargo check --locked --workspace
cargo check --locked -p rusty-gpui --examples
cargo test --locked -p rusty-gpui-theme
cargo test --locked -p rusty-gpui-gallery
cargo test --locked -p rusty-gpui-settings
cargo test --locked -p rusty-gpui --doc
```

The maintained fork retains upstream formatting where possible to keep synchronization reviewable. Do not reformat the entire fork just to change one component. Its upstream test-support suite is separate from the focused Rusty GPUI checks above.

Use the gallery for native review of relevant focus, keyboard, disabled, loading, error and scrolling states. Inspect representative light/dark themes and compact/spacious typography settings. Do not treat a compile check as native behavior validation.

Before a release, run the package/isolated-consumer checks in RELEASING.md. Dependencies between publishable packages must declare both a path for local development and a matching registry version. No library may depend on the gallery or on an unpublished Cargo patch.

## Build measurements

After caching dependencies, run `python3 scripts/measure-build.py --target-dir target/measurements/settings-1` to measure a fresh development build of the settings app. Choose a new target directory each time; the script never deletes caches. It records build time and executable size locally. These measurements exclude dependency downloads and do not measure optimized release size, startup latency or GPU performance.

## Dependency checks

Use `cargo tree` or `python3 scripts/dependency-report.py` to inspect dependencies. Run `cargo audit` for the workspace lockfile. After the package check, also run `cargo audit --deny unsound --file target/release-check/consumer/Cargo.lock`. These graphs differ: optional webview and test-support dependencies may have findings absent from the default consumer. Review findings without silently suppressing advisories.

The optional Linux/X11 checks in `tests/native_smoke.py` and `tests/native_regressions.py` require the tools listed in their headers. Their generated screenshots belong under `target/native-review`, outside Git.

## Upstream maintenance

rusty-gpui-components is derived from GPUI Component 0.5.1. Preserve its Apache license and attribution, record the source version in UPSTREAM.md and inspect the Rusty GPUI patch set when updating. It is published under its own name, not as an upstream release. Other packages in the source tree retain their existing licenses.
