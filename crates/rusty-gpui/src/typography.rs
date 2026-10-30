//! Named type roles derived from the configurable type scale.
use crate::{color, tokens};
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Debug)]
pub enum TextStyle {
    Display,
    Heading,
    Subheading,
    Body,
    Caption,
    Label,
}

pub fn text(value: impl Into<SharedString>, style: TextStyle, cx: &App) -> Div {
    let t = tokens(cx);
    let (size, weight, muted) = match style {
        TextStyle::Display => (t.type_scale[6], FontWeight::SEMIBOLD, false),
        TextStyle::Heading => (t.type_scale[5], FontWeight::SEMIBOLD, false),
        TextStyle::Subheading => (t.type_scale[4], FontWeight::MEDIUM, false),
        TextStyle::Body => (t.type_scale[2], FontWeight::NORMAL, false),
        TextStyle::Caption => (t.type_scale[1], FontWeight::NORMAL, true),
        TextStyle::Label => (t.type_scale[1], FontWeight::MEDIUM, false),
    };
    div()
        .font_family(t.font_family.clone())
        .text_size(px(size))
        .font_weight(weight)
        .text_color(color(if muted { t.muted } else { t.foreground }))
        .child(value.into())
}
