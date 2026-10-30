# Rusty GPUI 0.1.4

- Replaced duplicated per-helper sizing with a shared field foundation consumed by Input, Select and DatePicker, including raw constructors and nested popup search.
- Shared font family, size, line height, height, padding, gap, radius, border, surface, focus and disabled colors. Multiline inputs retain their editor height.
- Date styling applies to its actual trigger; empty date and select text use the same muted placeholder color. Disabled date triggers do not display a focus border.
- NumberInput composes the shared Input and matching square steppers. Added an interactive quantity example with validated nonnegative integers and increment/decrement events.
- Removed size-dependent right-padding overrides on inputs with suffix controls. Migrated gallery date, disabled and multiline fields to the facade.
- Added the field composition and ownership guide. Previous dropdown, scrolling, button, backdrop and disabled-input fixes remain included.

Validation: `cargo check --offline --locked --workspace` and formatting checks. No GUI automation or test suites run, following the requested quick handoff. Mac visual verification remains for the user.
