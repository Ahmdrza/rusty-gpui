//! Action variants compose the native Button foundation.
use crate::{RustyTheme, color, tokens};
use gpui::*;
use gpui_component::button::{Button, ButtonCustomVariant, ButtonVariants};
use rusty_gpui_theme::{Color, Density, ensure_contrast};

#[derive(Clone, Copy, Debug)]
pub enum ActionStyle {
    Primary,
    Secondary,
    Quiet,
    Danger,
    Accent,
    SecondaryAccent,
}

pub fn control_size(cx: &App) -> gpui_component::Size {
    match cx.global::<RustyTheme>().config.density {
        Density::Compact => gpui_component::Size::Small,
        Density::Comfortable => gpui_component::Size::Medium,
        Density::Spacious => gpui_component::Size::Large,
    }
}

/// A density-aware button with neutral primary styling by default.
pub fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    variant: ActionStyle,
    cx: &App,
) -> Button {
    let t = tokens(cx);
    let label = label.into();
    let icon_only = label.trim().is_empty();
    let button = Button::new(id)
        .label(if icon_only {
            SharedString::default()
        } else {
            label
        })
        .rounded(px(t.radius));
    match variant {
        ActionStyle::Primary => button.custom(
            ButtonCustomVariant::new(cx)
                .color(color(t.foreground))
                .foreground(color(t.background))
                .hover(color(t.muted))
                .active(color(t.muted))
                .border(color(t.foreground))
                .shadow(false),
        ),
        ActionStyle::Secondary => button.outline(),
        ActionStyle::Quiet => button.ghost(),
        ActionStyle::Danger => colored_button(button, t.danger, cx),
        ActionStyle::Accent => button.primary(),
        ActionStyle::SecondaryAccent => colored_button(button, t.secondary, cx),
    }
}

fn colored_button(button: Button, seed: Color, cx: &App) -> Button {
    let fill = ensure_contrast(seed, Color::WHITE, 4.6);
    let [l, c, h] = fill.oklch();
    let hover = ensure_contrast(
        Color::from_oklch((l - 0.04).max(0.0), c, h),
        Color::WHITE,
        4.6,
    );
    button.custom(
        ButtonCustomVariant::new(cx)
            .color(color(fill))
            .foreground(color(Color::WHITE))
            .hover(color(hover))
            .active(color(hover))
            .border(color(fill))
            .shadow(false),
    )
}
