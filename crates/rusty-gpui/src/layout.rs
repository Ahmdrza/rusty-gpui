//! Layout and surface primitives used by larger compositions.
use crate::{TextStyle, color, text, tokens};
use gpui::{prelude::*, *};
use gpui_component::StyledExt;
pub use gpui_component::foundation::SurfaceRole;

/// Vertical layout with the standard content gap.
pub fn stack(cx: &App) -> Div {
    div().v_flex().gap(px(tokens(cx).spacing[4])).w_full()
}

/// Wrapping horizontal layout with the standard control gap.
pub fn inline(cx: &App) -> Div {
    div().h_flex().flex_wrap().gap(px(tokens(cx).spacing[3]))
}

/// Surface presentation without anchoring, dismissal or focus behavior.
pub fn surface(role: SurfaceRole, cx: &App) -> Div {
    div()
        .border_1()
        .with_foundation(cx, |style, this| style.panel(this, role, cx))
}

pub fn field(label: impl Into<SharedString>, control: impl IntoElement, cx: &App) -> Div {
    div()
        .v_flex()
        .gap(px(tokens(cx).spacing[2]))
        .w_full()
        .child(text(label, TextStyle::Label, cx))
        .child(control)
}

/// Flat editorial section, without a decorative enclosing card.
pub fn section(
    number: &str,
    title: &str,
    description: &str,
    content: impl IntoElement,
    cx: &App,
) -> Div {
    let t = tokens(cx);
    div()
        .v_flex()
        .gap(px(t.spacing[5]))
        .py(px(t.spacing[6]))
        .border_b_1()
        .border_color(color(t.border))
        .child(text(format!("{number} / {title}"), TextStyle::Label, cx))
        .child(text(description.to_owned(), TextStyle::Caption, cx))
        .child(content)
}

pub fn empty_state(
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    cx: &App,
) -> Div {
    div()
        .v_flex()
        .items_center()
        .justify_center()
        .gap(px(tokens(cx).spacing[3]))
        .p(px(tokens(cx).spacing[7]))
        .child(text(title, TextStyle::Subheading, cx))
        .child(text(description, TextStyle::Caption, cx))
}
