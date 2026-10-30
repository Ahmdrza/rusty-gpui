# Component architecture

Rusty GPUI separates theme tokens, shared presentation, behavioral components and app compositions. Similar controls share foundations; distinct native behaviors remain in their own components.

## Dependency direction

`ThemeConfig → Tokens → ControlStyle + semantic palette → native components → app compositions`

`rusty-gpui-theme` has no renderer dependency. `rusty_gpui::apply_theme` maps generated tokens into one application-wide `ControlStyle` global and the upstream semantic palette. All participating components read that same global during rendering; `field::FieldStyle` is a compatibility alias for the same type, not a second set of settings.

The foundation lives in `crates/rusty-gpui-components/src/foundation.rs` so internal compositions inherit it too. Styling only facade helpers would leave popup search, menu triggers, close buttons and number steppers inconsistent. The versioned rusty-gpui-components dependency carries these changes into consumers; no application patch entries are needed.

## Shared foundations and consumers

| Foundation | Consumers | Owns |
|---|---|---|
| Control metrics and typography | Fields, actions, selection, navigation, surfaces, feedback, table | Font family/size, line height, gap, density-derived dimensions; minimum height keeps enlarged text from clipping |
| Field layout/surface | Input, Select trigger, DatePicker trigger; NumberInput through Input | Padding, single-line height, border/radius, focus/disabled visuals; multiline height remains independent |
| Action layout | Button text/icon/loading/variant builders; menu triggers, dialog actions, number steppers and close buttons through Button | Height, horizontal padding, compact spacing, square icon-only geometry; semantic variants retain their own fills |
| Selection row | Checkbox, Radio, Switch | Label typography, gap, minimum row height, alignment and disabled text; checkbox/radio indicator size and switch geometry derive from shared metrics |
| Item row | Popup menu items, ListItem, SidebarMenuItem | Typography, horizontal padding and row rhythm; separators retain their own geometry |
| Navigation control | Tab, segmented Tab variants and TabBar | Shared control/type scale; each tab variant retains its own selected/hover/border treatment |
| Surface roles | Generic popovers, menus, select/date popups, dialogs, sheets, notifications and public surface helper | Typeface, text/surface/border colors and corner roles; sheet edges remain square |
| Feedback content | Alerts and tags | Shared type and spacing; semantic tone and banner/pill/outline variants stay with the component |
| Data rows | Table header, virtual rows, stripe fillers, scrollbar offsets and loading skeleton rows | One shared height calculation, plus inherited typography, so visible and virtual geometry agree |
| Layout/type primitives | Gallery compositions, field labels, sections and empty states | Named text roles, stack/inline gaps, surface presentation |

## Public API modules

- `rusty_gpui::run`: single-window setup with bundled assets, initialization, Root, automatic overlay layers and last-window shutdown. The view renders its content. Explicit GPUI application setup remains available for custom application/window options.

- `rusty_gpui::actions`: action variants and the native Button adapter.
- `rusty_gpui::fields`: text, select, date and number adapters plus RustyField for shared labels, help and validation. Search/password compose Input.
- `rusty_gpui::typography`: named display, heading, body, caption and label roles.
- `rusty_gpui::layout`: stack, inline, surface, labeled field, section and empty-state compositions.

Existing root imports remain available. The gallery uses these modules through the prelude; the Foundations section displays the live metrics and the other sections exercise native component families.

## Behavior and customization

The foundation does not register clicks, duplicate keyboard handlers or implement its own text editor. Focus, selection, IME, menu navigation, table virtualization and dismissal remain in the native components. Existing focus-ring and semantic-theme utilities remain shared. No application domain appears in the foundation.

Configure the system with ThemeConfig rather than overriding each component. Density establishes the default control size; explicit small/large size categories are relative variants. Fields keep a common outer height; their size category also configures auxiliary popup content. Component-specific Styled overrides remain the escape hatch where the native builder supports them. `appearance(false)` keeps fields visually unframed.

Surfaces share presentation, not overlay geometry: dialogs center, sheets attach to viewport edges and popovers anchor to triggers. Their existing focus/dismissal code remains separate. SurfaceRole does not open or position anything by itself.

One theme per GPUI App is supported, matching the existing global theme model. Simultaneous themes in separate windows would require a separate scoped-theme design. Specialized widgets such as charts, dock layouts, rich editors and calendar cells continue to use their own native geometry and the shared semantic palette. They have not all received a dedicated Rusty GPUI presentation migration.

## Adding a component

1. Choose the existing family foundation or compose existing native controls. Add a new primitive only when the role cannot be expressed cleanly by one of those foundations.
2. Keep state and events in the behavioral component; apply shared presentation to the actual interactive element.
3. Derive geometry from tokens. Include nested controls and measurement/scroll calculations when migrating a family.
4. Add a real interactive gallery specimen. Inspect empty/filled, selected, focus, disabled and loading states as relevant, at multiple theme densities and font sizes.

See [support and limitations](support.md) for current platform and accessibility coverage. The runtime lacks a semantic accessibility tree and platform adapters for custom controls; see [support and limitations](support.md). Specialized widgets require further review.

## Assets and platform features

Use `rusty_gpui::Assets` as the application AssetSource. It bundles the 86 native icon paths referenced by the component library. Inter remains bundled and is registered by init. Apps with custom assets can delegate `icons/` paths to this source from their own AssetSource.

Rusty GPUI selects GPUI features by platform: macOS keeps the required font-kit backend and native Metal renderer; Linux keeps X11 and Wayland. The vendored component dependency no longer enables every GPUI default independently. Direct application GPUI dependencies should also disable defaults to retain this policy. No external asset crate is needed.

## Interaction consistency

Control focus policy is shared by buttons, checkboxes, radios, switches, selects and dates. Disabled controls drop focus and leave the tab order. Disabling an open select/date closes its popup. Select action handlers guard the disabled state. Switches now use the same native click dispatch path as other controls, which supports keyboard activation without registering an additional key handler. Root constrains Tab and Shift-Tab traversal to the top active dialog or sheet, including wrapping at its boundaries.

## Distribution boundary (0.4.0)

The gallery is an unpublished workspace app. The public entry point is rusty-gpui, supported by rusty-gpui-theme and the maintained rusty-gpui-components fork. Narrow rusty-gpui-runtime and rusty-gpui-http compatibility distributions eliminate the obsolete xattr dependency without Cargo patches; see each crate's UPSTREAM.md. Runtime source remains GPUI 0.2.2, but Rust type identity belongs to the Rusty GPUI package. Use rusty_gpui::gpui or the matching rusty-gpui-runtime dependency alias throughout an app.

Every first-party dependency has a version and local path in workspace.dependencies. Published manifests use registry requirements. Isolated consumer checks build from the actual crate archives with no gallery, path dependencies, patches, original GPUI/component packages or xattr. See RELEASING.md.
