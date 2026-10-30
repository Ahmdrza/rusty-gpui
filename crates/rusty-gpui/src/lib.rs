#![doc = include_str!("../README.md")]
pub use gpui;
pub use gpui_component as components;
pub use rusty_gpui_theme as theme;

use gpui::*;
use gpui_component::{Theme, ThemeMode};
use rusty_gpui_theme::{Color, Mode, ThemeConfig, Tokens, ensure_contrast};

pub mod assets;
pub use assets::Assets;
pub mod application;
pub use application::run;
pub mod actions;
pub mod fields;
pub mod layout;
pub mod typography;
pub use actions::{ActionStyle, button, control_size};
pub use fields::{FieldControl, RustyField, date_picker, input, number_input, select};
pub use gpui_component::foundation::ControlStyle;
pub use layout::{SurfaceRole, empty_state, field, inline, section, stack, surface};
pub use typography::{TextStyle, text};

#[derive(Clone, Debug)]
pub struct RustyTheme {
    pub config: ThemeConfig,
    pub tokens: Tokens,
}
impl Global for RustyTheme {}

/// Compatibility name from the project's pre-publication Form phase.
pub type FormTheme = RustyTheme;
/// Compatibility name from the project's pre-publication Form phase.
pub type FormField<C> = RustyField<C>;

pub fn color(color: Color) -> Hsla {
    rgb(color.to_u32()).into()
}
pub fn tokens(cx: &App) -> &Tokens {
    &cx.global::<RustyTheme>().tokens
}

pub fn init(cx: &mut App) {
    gpui_component::init(cx);
    cx.text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(
            include_bytes!("../assets/Inter.ttf").as_slice(),
        )])
        .expect("Unable to load bundled Inter font");
    apply_theme(ThemeConfig::default(), None, cx);
}

/// Apply the complete semantic palette to upstream controls and store Rusty GPUI tokens.
pub fn apply_theme(config: ThemeConfig, window: Option<&mut Window>, cx: &mut App) {
    let t = config.generate();
    Theme::change(
        if config.mode == Mode::Dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        },
        None,
        cx,
    );
    let theme = Theme::global_mut(cx);
    theme.font_family = t.font_family.clone().into();
    theme.font_size = px(t.font_size);
    theme.radius = px(t.radius);
    theme.radius_lg = px(t.radius + 3.0);
    theme.shadow = false;
    theme.tile_grid_size = px(config.spacing_unit.clamp(2.0, 8.0));
    let c = &mut theme.colors;
    c.background = color(t.background);
    c.foreground = color(t.foreground);
    c.border = color(t.border);
    c.input = color(t.border);
    c.muted = color(t.subtle);
    c.muted_foreground = color(t.muted);
    c.primary = color(ensure_contrast(t.primary, Color::WHITE, 4.6));
    c.primary_foreground = color(Color::WHITE);
    c.primary_hover = color(ensure_contrast(t.primary_hover, Color::WHITE, 4.6));
    c.primary_active = c.primary_hover;
    c.overlay = hsla(0., 0., 0., 0.42);
    // Upstream secondary also styles ghost text and neutral buttons. Keep it
    // neutral; the generated brand secondary is available as a separate variant.
    c.secondary = color(t.subtle);
    c.secondary_foreground = color(t.foreground);
    c.secondary_hover = color(t.border);
    c.secondary_active = color(t.border);
    c.accent = color(t.primary_subtle);
    c.accent_foreground = color(t.primary_text);
    c.selection = color(t.primary_subtle);
    c.ring = color(t.focus);
    c.caret = color(t.primary_text);
    c.popover = color(t.elevated);
    c.popover_foreground = color(t.foreground);
    c.sidebar = color(t.surface);
    c.sidebar_foreground = color(t.foreground);
    c.sidebar_border = color(t.border);
    c.sidebar_accent = color(t.primary_subtle);
    c.sidebar_accent_foreground = color(t.primary_text);
    c.sidebar_primary = color(t.primary);
    c.sidebar_primary_foreground = color(t.on_primary);
    c.tab_bar = color(t.background);
    c.tab_bar_segmented = color(t.subtle);
    c.tab_foreground = color(t.muted);
    c.tab_active = color(t.surface);
    c.tab_active_foreground = color(t.foreground);
    c.table = color(t.surface);
    c.table_head = color(t.subtle);
    c.table_head_foreground = color(t.muted);
    c.table_hover = color(t.subtle);
    c.table_row_border = color(t.border);
    c.list = color(t.surface);
    c.list_hover = color(t.subtle);
    c.list_active = color(t.primary_subtle);
    c.list_active_border = color(t.focus);
    c.list_even = color(t.surface);
    c.list_head = color(t.subtle);
    c.accordion = color(t.surface);
    c.accordion_hover = color(t.subtle);
    c.group_box = color(t.surface);
    c.group_box_foreground = color(t.muted);
    c.title_bar = color(t.background);
    c.title_bar_border = color(t.border);
    c.skeleton = color(t.subtle);
    c.progress_bar = color(t.primary);
    c.slider_bar = color(t.primary);
    c.slider_thumb = color(t.surface);
    c.switch = color(t.border);
    c.switch_thumb = color(t.surface);
    c.success = color(t.success);
    c.success_foreground = color(t.success.on_color());
    c.warning = color(t.warning);
    c.warning_foreground = color(t.warning.on_color());
    c.danger = color(t.danger);
    c.danger_foreground = color(t.danger.on_color());
    c.danger_hover = color(t.danger);
    c.danger_active = color(t.danger);
    c.info = color(t.primary);
    c.info_foreground = color(t.on_primary);
    c.link = color(t.primary_text);
    c.link_hover = color(t.primary_text);
    c.link_active = color(t.primary_text);
    cx.set_global(gpui_component::foundation::ControlStyle {
        height: t.control_height,
        padding_x: t.control_padding,
        font_size: t.font_size,
        font_family: t.font_family.clone().into(),
        radius: t.radius,
        gap: t.spacing[2],
    });
    cx.set_global(RustyTheme { config, tokens: t });
    if let Some(window) = window {
        window.refresh();
    }
}

/// The component catalog is provided through `components`. Reexporting upstream
/// types preserves their keyboard, editing, scrolling and focus behavior.
pub mod prelude {
    pub use crate::theme::{Color, Density, Mode, ThemeConfig};
    pub use crate::{
        ActionStyle, RustyField, RustyTheme, SurfaceRole, TextStyle, apply_theme, button, color,
        control_size, date_picker, empty_state, field, inline, input, number_input, section,
        select, stack, surface, text, tokens,
    };
    pub use crate::{FormField, FormTheme};
    pub use gpui_component::{
        ActiveTheme, Disableable, Icon, IconName, IndexPath, InteractiveElementExt, Root,
        Selectable, Sizable, StyledExt, WindowExt, h_flex, v_flex,
    };
    pub use gpui_component::{
        accordion::*,
        alert::*,
        avatar::*,
        breadcrumb::*,
        button::*,
        checkbox::*,
        date_picker::*,
        divider::*,
        input::{Input, InputEvent, InputState},
        menu::*,
        notification::*,
        progress::*,
        radio::*,
        select::*,
        skeleton::*,
        slider::*,
        spinner::*,
        switch::*,
        tab::*,
        tag::*,
    };
}
