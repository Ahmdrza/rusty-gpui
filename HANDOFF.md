# Rusty GPUI 0.4.0 handoff — 2026-10-30

The source is pushed to https://github.com/Ahmdrza/rusty-gpui and all five libraries are published as 0.4.0 on crates.io. This folder is now a Git checkout tracking origin/main.

Use [release readiness](docs/release-readiness.md) for current artifacts and next steps, [native review](docs/local-macos-review.md) for observed behavior, and [production readiness gates](docs/production-readiness.md) for outstanding work. Earlier compile-only cloud reports and historical screenshots are not the current validation record.

Recent fixes cover modal/drawer focus containment, header search first-character layout, a reproduced macOS activation deadlock, keyboard password reveal, table resize hit areas and gallery column identity after reordering. The gallery adapts to compact windows and includes native window presets and delayed picker disabling for reproducible review. The runtime still lacks semantic accessibility support for custom controls; do not describe this release as generally production ready.

There are five publishable libraries: HTTP compatibility, runtime compatibility, theme, components and facade. The gallery is unpublished. Applications use rusty-gpui and its matching runtime identity; no consumer Cargo patches are required. Preserve upstream licenses and provenance.

Run `bash scripts/run-macos.sh` for the gallery, or `cargo run --locked -p rusty-gpui --example minimal` for the independent starter. Full Xcode/Metal and current stable Rust are required. Preserve the existing target cache. Use RUSTY_GPUI_CONFIG_DIR for isolated review settings.

For future versions, follow RELEASING.md and regenerate package evidence after library changes using `python3 scripts/check-release.py --build-consumer`. Published versions are immutable; changes require a new version. GitHub macOS/Linux checks, dependency advisory auditing and packaged-consumer builds are configured. See release readiness for remote CI and docs.rs status. Windows remains experimental.
