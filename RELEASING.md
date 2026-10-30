# Releasing

All five libraries share one release version. The gallery has `publish = false`. Edit and publish from the same Git checkout; no separate release codebase is needed.

## Prepare

1. Update `workspace.package.version` and matching first-party requirements in the root Cargo.toml. Refresh Cargo.lock and add a changelog entry.
2. Follow the checks in [Contributing](CONTRIBUTING.md) and review relevant native interactions.
3. Verify the actual packages with Python 3.11+ and cached Cargo dependencies:

```sh
cargo fetch --locked
python3 scripts/check-release.py --build-consumer
```

The script stages a temporary local registry under `target/release-check`, checks package manifests/assets, and compiles a version-only consumer, README doctest and packaged examples. It uploads nothing. The generated report and archives are build artifacts; keep them out of Git. macOS/Linux CI runs these checks too.

First-party dependencies declare both a local path and matching registry version. Cargo removes local paths when packaging. Preserve explicit package include lists, bundled asset licenses and fork provenance. Use `cargo package --list -p rusty-gpui` to inspect contents.

## Publish

Authenticate using `cargo login`; keep credentials outside the repository. Commit and push the reviewed source. For each crate below, run `cargo publish -p NAME --dry-run`, then `cargo publish -p NAME`, and wait for it to become available before continuing:

1. `rusty-gpui-http`
2. `rusty-gpui-runtime`
3. `rusty-gpui-theme`
4. `rusty-gpui-components`
5. `rusty-gpui`

Do not bypass verification with `--no-verify`. A dependency must exist in crates.io before its dependents can be published there. Published versions are immutable; corrections require a new version.

## Verify

Build a fresh app using only the versioned crates.io dependency, without paths, patches or source replacement. Check the examples, hosted docs.rs pages and GitHub CI results. Date the changelog and create the matching `vX.Y.Z` tag and GitHub release. Include useful user-facing changes and current [support limits](docs/support.md) in release notes.
