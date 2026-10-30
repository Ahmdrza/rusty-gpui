# Local macOS review — 2026-10-28

This records the initial review followed by release preparation. The follow-up below supersedes the initial open items it explicitly covers.

Reviewed the extracted Rusty GPUI 0.4.0 source in `/Users/ahmadraza/Desktop/programming/rusty-gpui`. This directory has no `.git` history, so there is no commit or pre-existing diff to cite. Distribution candidates were not modified.

## Environment and build

- Apple Silicon (`arm64`), macOS 27.0.1 (26A434).
- Xcode 27.1 (27A9275), selected at `/Applications/Xcode.app/Contents/Developer`; `xcrun --find metal` found the installed Metal toolchain.
- Homebrew rustc 1.99.1 and Cargo 1.99.1.
- `bash scripts/run-macos.sh`: compiled and launched the gallery. The initial sandboxed fetch failed DNS resolution; the authorized network-enabled retry succeeded.
- `cargo build --locked --offline -p rusty-gpui --example minimal`: passed; starter also exercised natively.
- `cargo build --locked --offline -p rusty-gpui-gallery`: passed after the header change.
- `cargo fmt -p rusty-gpui-theme -p rusty-gpui -p rusty-gpui-gallery --check`: passed after formatting the header edit.

No broad test suite or packaging check was run. The change affects only gallery text, not published library APIs, dependencies or assets. Builds emitted existing float fallback warnings in runtime `taffy.rs`, deprecated `std::usize` warnings in component highlighting, and future compatibility notices for `block` and `proc-macro-error2`.

## Native observations

The bare Cargo executable was not discoverable through the UI tool. Temporary `.app` wrappers under `/tmp` allowed inspection of the compiled native gallery and starter. Gallery settings used `RUSTY_GPUI_CONFIG_DIR=/tmp/rusty-gpui-macos-review`; the user's default preferences were not overwritten. Screenshots were inspected directly through the native UI tool, not historical Linux harnesses.

| Area | Observed result |
| --- | --- |
| Text fields | Disabled gallery input rejected typing without a visible caret. Invalid email showed its error text and retained its red border while focused. |
| Shared sizing | Text, password, search, select, quantity and date triggers visibly shared single-line height. Comfortable, spacious and compact density were exercised; compact also inspected at 15px with Helvetica Neue, 10px radius and 6px spacing. Textarea retained a separate multiline height. |
| Project search | `Beacon` produced one matching result; `zzzz` showed zero and an empty message; clicking clear restored all five projects. |
| Searchable select | Typing `arch` reduced the popup to Archived. Escape dismissed the popup. |
| Quantity/date | Plus changed quantity from 1 to 2. Date popup stayed within the window near its lower edge; selecting October 28 displayed `2026/10/28`. |
| Menu | The full-width Project actions trigger opened a menu matching its width in dark appearance. |
| Actions | Light/dark variants were readable by visual inspection. Neutral primary reversed appropriately in dark appearance. Icon-only settings button was square. Loading showed a spinner and readable Working label. |
| Keyboard | Starting from focused header search, Tab reached Create workspace with a visible ring. Space changed action count 1→2; Enter changed 2→3. Tab and Shift-Tab traversal were observed. Pointer activation intentionally does not take button focus; the first pointer-then-key attempt therefore did not activate. No duplicate keyboard handler was added. |
| Modal | Empty OK submission stayed open and displayed Enter a workspace name. Entering Mac review and clicking OK closed it and displayed Created Mac review. Backdrop dimmed the full content viewport, including its top edge below the native titlebar. |
| Drawer | Opened flush to the top content edge with full backdrop coverage. Escape dismissed it. |
| Table | Wheel input over the 100-row table moved table rows while the surrounding page stayed fixed. Extra downward input at the last row remained contained. Wheel input outside the table moved the page normally. |
| Starter | Tab from the input reached Enable field. Space disabled the field and Save; attempted typing left Hello unchanged. Enter re-enabled it. Clearing the field showed Enter a value and disabled Save. Entering Mac review and clicking Save displayed Saved. |
| Settings save | Inspector changes wrote the expected isolated JSON: light, compact, Helvetica Neue, 15px type, 6px spacing and 10px radius. |

## Fix and limits

The running gallery still displayed `FORM / COMPONENT LIBRARY` above the main title. Updated that gallery-only label in `apps/gallery/src/main.rs` to `RUSTY GPUI / COMPONENT LIBRARY`. Rebuild and formatting passed. The rebuilt gallery was restarted, but repeated UI connection timeouts prevented visual confirmation of the new label and persisted theme after restart. The process was confirmed running; this is not a visual restart pass.

The native accessibility tree exposed only window chrome, not component semantics. Screen reader accessibility is not validated. Resize drag attempts produced no visible size change, so narrow-window layout, horizontal table scrolling, resize positioning, fullscreen and alternate display scales remain unverified. Modal focus trapping/restoration, outside-click dismissal, separate confirmation actions, table sorting/column resizing/large row sets, header search across categories, password reveal, long text and dynamic disable of open select/date were not completed. These are follow-up review items, not passes or reproduced defects. No production-readiness claim is made.

## Run again

From the source root:

```sh
bash scripts/run-macos.sh
cargo run --locked -p rusty-gpui --example minimal
```

To retain isolated review settings:

```sh
RUSTY_GPUI_CONFIG_DIR=/tmp/rusty-gpui-macos-review bash scripts/run-macos.sh
```

Nothing was committed, pushed, tagged or published in this review. Repository creation and crates.io publication remain later user-directed work.

## Release preparation follow-up

- Repository metadata is now `https://github.com/Ahmdrza/rusty-gpui` in all five normalized package manifests.
- Confirmed the corrected Rusty GPUI header and customized theme after a fresh app launch. The earlier UI reconnection timeout was resolved using a fresh temporary app wrapper.
- Reproduced a real focus escape: Tab after modal OK moved to All components behind the backdrop. Fixed shared Root traversal in `crates/rusty-gpui-components/src/root.rs` to constrain both directions to the top active dialog or sheet. Native recheck: input → Cancel → OK → input; Shift-Tab from input → OK; Escape restored the modal trigger. Drawer reverse traversal wrapped from Close to Done, and Enter on Done restored its trigger.
- Header search for `button` while Overlays was selected showed the Actions collection, confirming cross-category filtering.
- Confirmation OK closed the dialog and displayed Demo project archived.
- Table sort changed the first row from Atlas / 00001 to Evergreen / 00100. Switching to 10,000 rows retained descending order (Evergreen / 10000 first); scrolling moved the visible rows without moving the surrounding page.
- Fixed owned-source Rust warnings: explicit f32 grid literals in runtime `taffy.rs` and removed deprecated std::usize imports in component highlighting. Updated upstream provenance. Third-party future compatibility notices for block and proc-macro-error2 remain.
- Added a complete README app compiled as a doctest; fixed the ambiguous TextStyle glob import found by that check. README is also the facade's crate-level API documentation.
- Focused unit tests passed (5 theme, 5 gallery). Workspace check passed before the overlay change; the final overlay change compiled in the native gallery build and final packaged consumer. Facade/theme/gallery formatting passed.
- Final `python3 scripts/check-release.py --build-consumer` passed: all five actual archives, normalized manifests/assets, registry-only dependency resolution, custom component macro API, full packaged starter, and README doctest. The independent app's only dependency was `rusty-gpui = "=0.4.0"`; no paths, patches or gallery dependency.
- Launched that final packaged executable through a temporary app wrapper: it rendered correctly, accepted Packaged app as its value, and Save displayed Saved. This is native evidence from the distribution artifacts, not the workspace starter.
- Documentation generated from the packaged facade/theme with broken intra-doc links denied. Hosted docs.rs and GitHub Actions runs remain unverified until publication/push.

Checksums and exact package evidence are in `release-validation-0.4.0.json`; distributable candidates are under `target/package` and bundled in `target/release-check/rusty-gpui-0.4.0-candidates.zip`. The original sibling candidate folder was preserved.

Remaining review gaps: native window resize gestures did not change the window, so narrow-window/horizontal-scroll/alternate-scale conditions remain unverified. Column resize, password reveal/long text, outside-click dismissal and dynamic disable of open select/date were not completed. Screen reader accessibility remains unverified: the automation tree exposes only native window chrome. These limits are not claimed as passes and do not constitute reproduced defects. This is preparation for an early pre-1.0 release, not blanket production certification.

## Header search first-character regression

Reproduced the user's report after selecting Inputs: typing `g` filtered collections but hid the character and placed the clear button beside the leading icon; typing `h` then displayed `gh` and restored the button to the right. The gallery header search container had a maximum width without an explicit width. Added `w_full()` alongside its existing 500px maximum in `apps/gallery/src/main.rs` so its percentage-sized input has a defined available width during the first filtered layout.

Native recheck of the rebuilt macOS gallery passed: Inputs → search → `g` displayed immediately with the clear button at the right; `gh` displayed correctly with the empty state. After switching to Overlays, single-character `z` and `q` queries displayed immediately, with matching and empty results respectively. Backspace and the clear button worked; clearing restored Overlays and the placeholder. Gallery build and formatting check passed. This fix changes only the unpublished gallery; the five library archives and their recorded hashes are unchanged.

## Extended review — 2026-10-28–29

This section supersedes the earlier gaps it explicitly covers. Final library archives are regenerated after these changes; earlier checksums do not validate the new runtime and component sources.

- **macOS activation:** reproduced an unresponsive gallery during activation. A process stack sample showed `window_did_change_key_status` calling `resignKeyWindow` while holding the window-state mutex; AppKit synchronously re-entered that handler and waited for the same mutex. Released the guard before the AppKit call. A fresh native build activated successfully and remained responsive through compact/wide resizing.
- **Compact layout:** added native window presets (600×650 and 1440×900 logical pixels). Below 850px, category navigation wraps above the content; below 1100px, the theme inspector is available inline through its toggle. Native 600px and 800px layouts, category navigation, search, light/dark inspector changes, and inspector show/hide were exercised. An initial inspector-sheet implementation hit a render ownership error during development; it was replaced with the tested inline composition.
- **Password:** the original reveal control required a held mouse button and was excluded from keyboard traversal. Changed it to a native keyboard-focusable Show/Hide toggle. Native Tab → Enter revealed `Demo-password-123`; Space masked it again without duplicate activation. Long text remained clipped within the field, with keyboard navigation reaching its end.
- **Dynamic picker disabling:** added a gallery-only delayed-disable demonstration. Opened the searchable select, then observed it close when disabled; pointer/Enter did not reopen it. Re-enabled the pickers, opened the date calendar and observed it close when disabled. Both triggers showed the disabled surface and the explicit disabled status.
- **Outside click:** the confirmation-style workspace modal intentionally remained open on backdrop clicks (`confirm()` disables backdrop dismissal). Escape closed it. The drawer closed on a backdrop click. Rechecked modal/drawer geometry and drawer backdrop dismissal at 600px width on October 29.
- **Table columns:** header dragging exposed a gallery data-association bug: the table reordered its column groups while the delegate still chose headers/cells by their previous positions. Implemented delegate reordering and cell lookup by stable column key. Native recheck moved Status after Owner; each header retained the corresponding values. Added a regression test covering reorder plus the 10,000-row descending dataset.
- **Column resizing and horizontal geometry:** enlarged the divider hit area from 2px to 6px while retaining a thin painted border. Native dragging visibly widened the first column. At 600px, Right-arrow column navigation revealed the clipped Owner column; header and body shifted together while surrounding page content stayed fixed. Horizontal wheel requests through the automation tool produced no visible movement, so trackpad/wheel horizontal scrolling remains unverified rather than being claimed as a pass.
- **Final focused checks:** native gallery build and formatting passed. Six gallery tests and five theme tests passed on October 29, including the new column-identity regression. Workspace compilation passed after the runtime activation and password changes; final gallery/table changes compiled in the final gallery build. Current evidence/logs live under `target/review`.

**Accessibility finding:** custom controls are absent from macOS's accessibility tree, and the runtime has no semantic accessibility-tree/platform adapter implementation. This is a real missing capability, not just an incomplete test. Keyboard improvements do not establish screen-reader support. See [production readiness gates](production-readiness.md).

Still unavailable locally: fresh native Linux/Windows execution, other display scales/monitors, hosted GitHub CI before a push, and hosted docs.rs before publication. The Linux/macOS package workflow now runs on main pushes and pull requests, with independent consumer builds for each OS. These runs remain pending; Windows stays experimental. Nothing has been committed, pushed, tagged or published.

### Final distribution and manual trackpad confirmation — 2026-10-29

The user manually confirmed that horizontal two-finger scrolling over the narrow table moves its columns while the surrounding page stays still. This closes the trackpad review gap above as user-reported native evidence; the automation's horizontal gesture limitation remains separate.

Regenerated all five `.crate` archives from the final library sources. The isolated registry consumer, README doctest and full starter build passed. Launched the final packaged starter, changed its value to `Final packaged app` and observed the Saved notification. Packaged facade/theme documentation generated successfully with broken intra-doc links denied. Current checksums are in `release-validation-0.4.0.json`. Added a dependency-advisory CI job; its first remote execution remains pending with the other GitHub workflows.

RustSec cargo-audit 0.22.2 found zero vulnerability-classified findings in the workspace and packaged-consumer lockfiles. The packaged consumer passed with unsoundness advisories denied; 14 maintenance warnings remain. The full workspace lock additionally has optional webview/test-support unsoundness findings and 16 maintenance warnings. See [dependency audit](dependency-audit.md) for IDs, dependency paths and exact scope; this is not an all-features security pass.


### Simple application setup and packaged examples — 2026-10-30

Added `rusty_gpui::run` to install bundled assets/fonts, initialize the default theme, wrap the view in Root and render sheet/dialog/notification layers. The helper requests a centered 720×520 logical window clamped to the primary display and quits on last-window close. The automation session presented an expanded window, so exact initial window geometry is not recorded as a native pass.

The hello, form and overlays examples are complete copyable files using the facade dependency alone. The form was checked natively for required-name validation, editing, checkbox state in the Save notification (`Saved Quick app; updates on`), and disabled input/Save when its switch is off. That form interaction check preceded the final window-bounds-only adjustment. From the final packaged consumer, opened the confirmation dialog, selected OK and observed `Confirmed`; opened the Details drawer and selected Done, which removed it. Closing the final overlay window removed its app from the running-app inventory, verifying helper shutdown.

The final five archives passed the isolated registry consumer check, README doctest and builds of the starter and all packaged examples. Facade/theme/gallery formatting and packaged facade/theme documentation (broken intra-doc links denied) passed. The packaged consumer again passed cargo-audit with unsoundness denied: zero vulnerabilities, zero unsoundness warnings and 14 maintenance warnings. Existing platform/accessibility limits remain.

The local release checker now exposes only dependency versions with cached archives, avoiding sparse-index entries whose archives are unavailable offline. It also checks/builds the example files extracted from the actual facade archive. This tooling change does not alter published dependency requirements.
