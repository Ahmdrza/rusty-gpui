# Current release: 0.1.3

Native build checked on Linux only. No GUI testing or test suites run for this update, as requested. The record below describes earlier revisions.

# Current release: 0.1.2

Native build passes on Linux. At the user’s request, no GUI tests or test suites were run for 0.1.2. macOS runtime review is left to the user. The disabled-state changes are compiled but still need their visual/focus review.

The following record describes earlier revisions, not a full verification of 0.1.2.

# Validation record

Verified in the cloud on 2026-10-28 using Debian 13 and Rust 1.99.1:

- Native workspace builds and type-checks with the committed Cargo.lock.
- Eight Rust tests pass: color parsing, OKLCH conversion and monotonic scales, contrast across extreme seeds in both appearances, spacing/type sizing, explicit secondary colors, settings round trips and bounds, and complete 10,000-row table sorting.
- Formatting and Clippy checks pass for first-party code. Vendored GPUI Component emits deprecation warnings, and a transitive macro crate reports a future-compatibility warning.
- The native window runs under X11 with software Vulkan (Mesa llvmpipe).
- Native XTest smoke checks exercise category selection, action callbacks, disabled actions, Tab/Space keyboard activation without duplicate dispatch, checkbox/switch/radio changes, tabs, pagination, modal and drawer opening/Escape dismissal, 10,000 table rows, frame sampling and dark mode.
- Native review captures are included: light appearance, modal, performance lab and dark appearance.
- The macOS shell launchers pass shell syntax checks. A macOS CI workflow is supplied, but it has not been run in this session.

The 120-frame sampler completed in the debug/software-rendered environment. Its observed callback mean was approximately 87 ms and p95 106 ms. These are diagnostic cloud observations with smoke instrumentation enabled, not a prediction or target for your Mac. Measure without `RUSTY_GPUI_SMOKE_STATE` on your own hardware.

macOS compilation, Metal rendering, Retina scaling, macOS font metrics and VoiceOver behavior remain to be checked on the laptop. The source archive contains no prebuilt macOS binary.
