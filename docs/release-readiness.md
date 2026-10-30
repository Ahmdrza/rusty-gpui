# Rusty GPUI 0.4.0 release

Released on 2026-10-30: source is on [GitHub](https://github.com/Ahmdrza/rusty-gpui), and all five libraries are available on crates.io. This is an early pre-1.0 release with documented platform and accessibility limits.

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

Published archive copies and checksums: `target/published-release/`. These were downloaded from crates.io and verified against registry checksums. Older candidate bundles are preparation snapshots; use the published archives or the v0.4.0 GitHub release.

Evidence: [release-validation-0.4.0.json](release-validation-0.4.0.json) and [native review](local-macos-review.md). The original sibling candidate folder was preserved as historical preparation evidence.

## Publication verification

- All five crates passed normal Cargo dry-run and publication verification, in dependency order; `--no-verify` was not used.
- A fresh consumer with only `rusty-gpui = "=0.4.0"` built its app and hello/form/overlays examples from crates.io. Metadata confirmed registry dependencies throughout, without paths, patches or source replacement.
- The registry-built hello app launched natively on macOS, and its button displayed the Hello notification.
- RustSec audit of this fresh lockfile passed with unsoundness denied: zero vulnerabilities, zero unsoundness warnings, 14 maintenance warnings.
- Hosted docs.rs API pages are available for all five crates.
- The initial source push passed both [Native checks](https://github.com/Ahmdrza/rusty-gpui/actions/runs/41105530321) and [Package release candidate](https://github.com/Ahmdrza/rusty-gpui/actions/runs/41105530440), including macOS/Linux builds. Later documentation pushes run the same workflows; check the Actions tab for their current status.

## Limits to retain in release communication

The runtime lacks semantic screen-reader support for its custom controls. This blocks a general production-ready recommendation; see [production readiness gates](production-readiness.md). Windows and alternate display scales remain unverified. Linux native visual/keyboard execution remains pending despite passing Linux CI builds; this Mac cannot establish it. Horizontal trackpad scrolling was manually confirmed by the user on October 29; other native details are in the review. Dependency maintenance warnings, optional webview/test-support unsoundness advisories, and third-party future compatibility notices remain. None of the remaining gaps should be represented as validated behavior.
