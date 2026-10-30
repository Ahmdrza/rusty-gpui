# Rusty GPUI 0.2.0

Expanded the field foundation into reusable presentation primitives across the main component families.

- One ControlStyle owns shared typography and geometry. FieldStyle is a compatibility alias.
- Text/icon/loading buttons, selection rows, menu/list/sidebar items, tabs, popup and overlay surfaces, alerts/tags and table rows consume the shared foundation.
- Internal compositions inherit the foundation automatically: popup searches, menu triggers, close buttons, dialog actions and numeric steppers.
- Table headers, virtual rows, fillers, scrollbar offsets and loading rows share one height calculation.
- Split the facade into actions, fields, typography and layout modules, preserving root imports. Added reusable stack, inline and surface helpers and migrated gallery composition to them.
- Added a live foundation summary and architecture guide with ownership, extension rules and explicit boundaries.

Validation: workspace compile and formatting checks. No GUI automation or test suites run for this release. Native macOS review remains with the user. Existing images and older test reports describe earlier releases.
