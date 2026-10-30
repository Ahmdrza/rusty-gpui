# Native component catalog

The native gallery demonstrates the component families below.

| Family | Interactive gallery specimens |
|---|---|
| Foundations | Display, heading, subheading, body, caption, labels, generated color scale, spacing scale, contrast readout |
| Actions | Neutral primary, outline, ghost, primary accent, generated secondary accent, destructive, icon, disabled, loading, size variants, tooltip |
| Inputs | Text, email with validation hint, masked password with reveal, multiline text, live search with filtered projects and an empty state, disabled field, searchable select, date picker, validated numeric quantity with step events |
| Selection | Checkbox, disabled checkbox, radio group, switch, segmented control, slider |
| Navigation | Underline tabs with content, breadcrumbs, dropdown actions menu, bounded pagination, category sidebar |
| Overlays | Modal with validation, confirmation dialog, side drawer, tooltip, success notification |
| Feedback | Status tags, info/success/warning/error alerts, spinner, skeleton, progress |
| Data & structure | Virtualized table, column resizing, ascending/descending name order, avatars, accordion, empty state, labeled divider |
| Performance | 100/1,000/10,000 table rows, 120-frame callback sampler, mean and p95, interaction history |

## Additional native primitives

Available through `rusty_gpui::components`, but not all have dedicated Rusty GPUI gallery examples yet:

- Date ranges, calendar, color picker, input masks and code editing.
- Popover, links, keyboard shortcut badges, clipboard controls and collapsible regions.
- Tree and virtual lists, description lists, group boxes and forms.
- Sidebar primitives, resizable panels, docking, split layouts and window title bars.
- Charts, plot primitives and richer text rendering.

Use their upstream builder APIs and Rusty GPUI tokens. These controls should be evaluated in the context of the apps that use them; “available” does not imply that every variant has received Rusty GPUI-specific visual review.

## Theme contract

- Supply a primary seed, optional secondary seed, appearance, density, font family/size, base spacing unit, radius and neutral tint.
- A secondary seed is related by a 60-degree hue shift with reduced chroma when none is supplied.
- Palettes use OKLCH with chroma reduction for out-of-gamut colors.
- Text on tinted backgrounds, muted text and semantic colors are contrast-adjusted against their intended surface, with a small rounding margin.
- The primary seed is preserved in the theme tokens. Filled action colors are adjusted for readable white text; text accents are adjusted independently.
- Light and dark are generated independently. Status hues retain their semantic identity.
- Components use semantic tokens. No app domain is assumed.

See [component architecture](architecture.md) for the shared foundations used by the main families and the boundaries for specialized widgets.
