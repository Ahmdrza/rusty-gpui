# Shared field foundation

Initialize Rusty GPUI once and apply ThemeConfig when it changes. There is one field style per GPUI App, matching the existing application-wide theme model. Every participating field reads the current style at render time, including nested search inputs.

## Ownership

- `rusty-gpui-theme::Tokens` derives control height, horizontal padding, font size, font family, spacing and radius from the configuration.
- `rusty_gpui::apply_theme` installs these metrics in `gpui_component::foundation::ControlStyle` (`field::FieldStyle` remains an alias) and maps semantic colors onto the component theme.
- `FieldStyle::layout` owns typography, line height, padding, gaps and single-line height constraints. Vertical padding accounts for the 1px border. Multiline inputs retain their requested height.
- `FieldStyle::surface` owns radius, surface, text, border and disabled/focus colors. The disabled state takes precedence over focus.
- Input, Select and DatePicker call both functions on their actual field elements. NumberInput composes Input and uses the shared height for its square stepper buttons.
- Each control retains its native state, focus, keyboard, text editing/IME, selection, popup and event behavior. Presentation reuse does not substitute a text editor for a calendar or select trigger.
- `rusty_gpui::field` groups a label and a control; it does not add another border or padding shell.

## Composition

```rust,ignore
field("Name", input(&name_state, cx), cx);
field("Search", input(&search_state, cx)
    .prefix(Icon::new(IconName::Search)).cleanable(true), cx);
field("Password", input(&password_state, cx).mask_toggle(), cx);
field("Project", rusty_gpui::select(&select_state, cx), cx);
field("Date", date_picker(&date_state, cx).cleanable(true), cx);
field("Quantity", number_input(&number_state, cx), cx);
field("Description", input(&multiline_state, cx).h(px(100.)), cx);
```

The states must be initialized with the appropriate behavior: multiline mode on InputState, a SearchableVec delegate for searchable selects, and NumberInputEvent::Step handling for numeric increment/decrement. The gallery shows each. NumberInput emits step events rather than assuming business rules, limits or precision.

Raw Input/Select/DatePicker constructors also inherit this foundation. Helpers select a density category for auxiliary UI. Explicit Styled overrides apply after the field defaults; DatePicker overrides now style its actual trigger. Prefer ThemeConfig for consistent app-wide changes. Code editors keep their separate editor metrics; other migrated component families consume the same ControlStyle, with family-specific layout methods. See architecture.md.

## Review on macOS

In Inputs, compare name, disabled, password, search, select, quantity and date. Change density, font size, font family, spacing and radius in the inspector. Their single-line field heights and typography should stay aligned; description remains multiline. Check empty/filled, focus, clear buttons, disabled cursor and popup selection. The shared implementation has been compile checked in cloud; this release has not been visually verified on macOS.

## Validation and supporting text

`RustyField` reuses the labeled-field layout and adds help text, a required marker and an optional error. Errors replace help text and set the control's invalid presentation; validation policy remains in your app. The message is text, so error communication does not depend on color alone.

```rust,ignore
RustyField::new("Email", input(&email_state, cx))
    .required(true)
    .description("Used for account notifications.")
    .error(invalid.then(|| "Enter a valid email address.".into()))
```

Input, Select, DatePicker and NumberInput implement FieldControl. They also accept `.invalid(bool)` when composed without RustyField. Error borders remain visible during focus; disabled controls retain their disabled border. NumberInput applies the error treatment to its center and stepper edges. Do not use `InputState::validate` for presentation errors: that callback rejects edits and is intended for input constraints such as numeric-only entry.

`required(true)` marks the label; it does not automatically block submission. Pass `.disabled(true)` to the actual control. Native label/message associations for assistive technology remain subject to GPUI's platform support and require native review.
