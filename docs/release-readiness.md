# Rusty GPUI 0.4.0 release preparation

Local release gates passed on Apple Silicon macOS with Rust 1.99.1. Prepared as an early pre-1.0 release with documented platform and review limits. Nothing has been pushed or published.

## Delivered

- All five crates contain the selected repository URL: https://github.com/Ahmdrza/rusty-gpui.
- Complete copyable quickstart, compiled as a doctest and included in API documentation.
- Added `rusty_gpui::run` for single-window setup, three small runnable examples (hello, form and overlays), and a [guide for Node.js developers](coming-from-node.md). Examples remain in the same repository and are checked from the packaged facade.
- Fixed modal/drawer keyboard focus escaping into the background; verified native wrapping and focus restoration.
- Current Rust warnings fixed in maintained source; upstream provenance preserved.
- Focused unit tests, workspace compilation, formatting and packaged documentation checks passed.
- Actual `.crate` archives verified with a separate version-only consumer, full starter build and native launch/edit/save check. No consumer patches, paths or gallery dependency.
- Candidate workflow now builds the packaged starter and exports checksum evidence. Native workflow now checks the quickstart doctest.
- Fixed a reproduced macOS activation deadlock, keyboard-enabled password reveal, and table column identity after reordering; verified compact layouts, column resizing, keyboard horizontal scrolling and dynamic picker disabling.
- Package checks now run automatically on main pushes and pull requests for macOS and Linux.
- RustSec found zero vulnerability-classified advisories. The packaged consumer passed with unsoundness advisories denied; dependency maintenance warnings and optional/test-feature findings remain documented in [dependency audit](dependency-audit.md).

Candidate archives: `target/package/rusty-gpui*.crate`.

Bundle: `target/release-check/rusty-gpui-0.4.0-candidates.zip`.

Reviewed source snapshot for GitHub handoff: `target/release-check/rusty-gpui-0.4.0-reviewed-source.zip` (includes source checksums; excludes build caches and local review tools).

Evidence: [release-validation-0.4.0.json](release-validation-0.4.0.json) and [native review](local-macos-review.md). The original sibling candidate folder contains the older handoff archives and was preserved; use the newly generated files above.

## Remaining publication steps

1. Put this reviewed source in the selected GitHub repository and run its Native checks and Package release candidate workflows. This extracted directory currently has no Git history. No remote CI run has been verified locally.
2. Authenticate Cargo with the intended crates.io owner account, keeping credentials outside this repository. The five names returned 404 from the crates.io API on 2026-10-28; that does not reserve them.
3. Follow [RELEASING.md](../RELEASING.md): dry-run and publish each crate individually in order HTTP → runtime → theme → components → facade. Wait for each dependency to appear before the next. Do not use `--no-verify` for real publication.
4. Verify a fresh app against crates.io and check hosted docs.rs builds. Then date the changelog and create the v0.4.0 tag/release.

## Limits to retain in release communication

The runtime lacks semantic screen-reader support for its custom controls. This blocks a general production-ready recommendation; see [production readiness gates](production-readiness.md). Windows and alternate display scales remain unverified. Fresh Linux native execution and remote CI are pending; this Mac cannot establish them. Horizontal trackpad scrolling was manually confirmed by the user on October 29; other native details are in the review. Dependency maintenance warnings, optional webview/test-support unsoundness advisories, and third-party future compatibility notices remain. None of the remaining gaps should be represented as validated behavior.
