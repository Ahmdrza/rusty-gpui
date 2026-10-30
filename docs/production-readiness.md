# Production readiness gates

Rusty GPUI 0.4.0 is a published early pre-1.0 release. Passing compilation and package checks does not establish production readiness. This file separates release engineering from the remaining renderer and platform work.

## Accessibility blocks a general production recommendation

The current GPUI 0.2.2 runtime has no semantic accessibility tree or platform adapter for custom-rendered controls. On macOS the gallery exposes its native window chrome, but its fields, buttons and other controls are absent from the accessibility tree. This is an implementation gap, not merely an unperformed screen-reader test. Keyboard focus and tooltips do not substitute for that tree.

Completion requires all of the following:

- A runtime semantic tree with stable element identities, roles, names, bounds, state, focus and supported actions, updated with each rendered frame.
- Platform adapters for macOS accessibility and Linux assistive technology, with actions routed through the existing input/focus system.
- Component semantics for labels, required/invalid/disabled fields, password protection, selection state, expanded popups, table headers/rows and modal boundaries.
- Native screen-reader checks of navigation, editing, value changes and validation; modal background content must be excluded while open. Virtualized table semantics must remain valid while rows are recycled.

These changes span the runtime, element API and component catalog. They are not implemented by this release preparation and must not be advertised as supported. Applications that require screen-reader support should not adopt this release for production.

## Platform and release evidence

- macOS: local native results and their exact limits are recorded in [the review](local-macos-review.md).
- Linux: compilation, tests and packaged-consumer builds passed in the initial GitHub CI runs. A native Linux visual/keyboard review is still required; a macOS run cannot establish Linux runtime behavior.
- Windows: experimental, outside the validated release targets. Windows support requires its own build and native review before being advertised as supported.
- Display scaling: the local Retina run does not establish behavior on other display scales or monitors.
- Distribution: all five archives are published, a fresh registry consumer built and ran on macOS, and hosted API docs are available. Future library changes require a new version and refreshed package evidence.

## Dependency maintenance and optional features

The October 29 RustSec scan found no vulnerability-classified advisories in either lockfile. The packaged consumer also passed with unsoundness advisories denied, but retained 14 unmaintained-package warnings. The full workspace lock retained 16 maintenance warnings and four unsoundness advisories: two in git2 0.20.4 through test-support features, and glib 0.18.5 / rand 0.7.3 through the optional webview dependency. These graphs are absent from the verified facade consumer. This is not an all-features security pass.

The optional webview feature remains experimental and must not be described as production validated. Its dependency migration needs Linux GTK/WebKit build and native verification. Upstream dependency maintenance remains ongoing work even where the current audit has no vulnerability findings. Exact scan metadata, affected versions and graph distinctions are recorded in [dependency audit](dependency-audit.md).

## GitHub handoff

Both Native checks and Package release candidate run on push/PR for macOS and Linux. The packaging job builds an independent application from the actual crate archives. Both workflows passed for the initial source push; see release readiness for links. Later documentation pushes run them again. Neither workflow publishes crates.

Before describing a release as production ready, close the accessibility gap, obtain the platform evidence above, and evaluate a real application's editing, navigation, overlays and data workflows. See [release readiness](release-readiness.md) for the artifact and publication steps.
