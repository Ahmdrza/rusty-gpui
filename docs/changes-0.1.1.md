# Rusty GPUI 0.1.1

This revision addresses the first native review.

- **Nested table scrolling:** The table's overlay routes vertical wheel/trackpad deltas to its row viewport and horizontal deltas to its column viewport. Offsets clamp at the viewport's limits. Wheel gestures inside the table stay there; gestures outside it still scroll the gallery. Table overlays are positioned relative to the table rather than the whole page, and its example viewport retains its intended height.
- **Search:** The input example now filters a visible project list, including a no-results state and restoration when cleared. The library search finds collections across all categories, trims whitespace and accepts multiple terms. Selecting a category clears that search and returns to the top of the page.
- **Drawer:** The default sheet top margin is zero, matching an app with a native title bar. The panel and backdrop cover the client area from its top edge. Apps that draw a custom title bar can explicitly set `margin_top` to that bar's height.

The data and theme APIs remain compatible with 0.1.0. Native geometry/scrolling corrections apply through the vendored component dependency, so bring the updated `vendor` folder with the new source.

Extract the updated source into a fresh directory to preserve any local source edits. Saved theme settings use the same location and schema as before. Run `bash scripts/run-macos.sh` from the extracted `form` folder.
