# Native dependency audit — Rusty GPUI 0.3.0

The count displayed by Cargo during a first build is not a count of installed applications. Dependency versions, host build tools, generated code, features and build profiles all affect compilation work. Cached artifacts usually make subsequent builds faster.

We compared the 0.2.0 source archive with 0.3.0 using the locked graph, `cargo tree --target aarch64-apple-darwin --edges normal,build`, deduplicating package name/version pairs. The count includes Rusty GPUI's three workspace packages and host build/procedural macro dependencies, and excludes dev dependency edges. It is not a binary-size or build-time measurement.

| Apple Silicon graph | Package versions |
|---|---:|
| Rusty GPUI 0.2.0 | 512 |
| Rusty GPUI 0.3.0 | 493 |
| Reduction | 19 |
| Current GPUI subtree | 431 |
| Current packages outside GPUI | 62 |

The current Intel Mac graph contains 495 unique package versions. Counts are generated on Linux by selecting Mac targets; this does not compile or run the Mac app. The earlier metadata-only exploratory counts include feature unification across targets and are not this platform-selected comparison.

## Changes

- Disable GPUI defaults at the workspace and vendored-component dependency edges, so they cannot independently enable every backend.
- On macOS explicitly retain font-kit. GPUI substitutes a NoopTextSystem without it, so removing it is not a viable optimization. Native Metal remains the renderer.
- Keep both X11 and Wayland on Linux; keep the Windows manifest feature on Windows.
- Bundle the existing 86 icons in Rusty GPUI's Assets provider. Remove the separate gpui-component-assets dependency and the folder-path interpolation feature it enabled.
- Keep the existing widget catalog and editing behavior. Rich text, syntax support, images and native services still account for substantial transitive weight; stripping them would require a deliberate feature/API design rather than deleting arbitrary crates.

No cold-build speed or executable-size improvement is claimed: those need measurements on the laptop. Adding another dependency that enables GPUI defaults can increase this graph again because Cargo features are additive.

## Reproduce

```sh
python3 scripts/dependency-report.py
python3 scripts/dependency-report.py --target aarch64-apple-darwin --json
python3 scripts/dependency-report.py --why rusty-gpui-runtime
```

The script defaults to locked, offline analysis. After an initial dependency fetch this should need no network. If registry data is missing, `--online` lets Cargo fetch it. The checked-in dependencies-apple-silicon.json lists the exact current package versions and version duplication.

## 0.4.0 distribution update

The earlier table is the historical 0.3.0 comparison. The current release candidate uses named Rusty GPUI runtime/HTTP/component compatibility packages and removes xattr entirely. Regenerate the current snapshot with dependency-report.py. Subtree counts are taken from the same app-selected graph; selecting the runtime alone can activate different default features and is not a valid comparison.

## RustSec advisory scan — 2026-10-29

Audited the workspace Cargo.lock and the independent consumer generated from the final package archives using cargo-audit 0.22.2. Database commit: `ee1a49a2769a24a596afc1c20cb33748d2769586` (1,386 advisories; database update 2026-10-29).

| Graph | Vulnerability findings | Unsoundness warnings | Maintenance warnings |
| --- | ---: | ---: | ---: |
| Full workspace lock, including optional/test dependencies | 0 | 4 | 16 |
| Packaged facade consumer | 0 | 0 | 14 |

The packaged consumer passed `cargo audit --deny unsound`. This does not prove the absence of unknown defects or establish all-features safety. The optional webview graph is excluded from that consumer.

Workspace-only unsoundness findings:

- git2 0.20.4: RUSTSEC-2026-0183 and RUSTSEC-2026-0184, patched in 0.21.0+. Reverse dependency analysis traced these to gpui_util with test-support features.
- glib 0.18.5: RUSTSEC-2024-0429, patched in 0.20.0+. Reached through optional lb-wry → GTK/WebKit.
- rand 0.7.3: RUSTSEC-2026-0097; patched release lines include 0.8.6+, 0.9.3+ and 0.10.1+. Reached through optional lb-wry → kuchikiki → selectors → phf build dependencies. The advisory concerns thread-RNG use through a reentrant custom logger; it is retained as a finding rather than silently ignored.

The consumer's unmaintained-package warnings cover async-std, instant, paste, proc-macro-error2, rustls-pemfile, two rustybuzz versions, three ttf-parser versions, unsafe-libyaml, zbus_names, zvariant and zvariant_derive. Replacement requires coordinated upstream migrations, including rendering/font and Linux service dependencies. The workspace additionally includes fxhash and proc-macro-error warnings. No advisory IDs were suppressed.

Raw local results: `target/review/audit.json` and `target/review/consumer-audit.json`. The GitHub workflows audit both the workspace lock and packaged consumer; the consumer job denies unsoundness findings. Remote results remain pending until push. Refresh these scans before publication because advisory data changes over time.
