//! Field adapters preserve editing and popup behavior.
use crate::control_size;
use crate::{TextStyle, color, text, tokens};
use gpui::{prelude::*, *};
use gpui_component::{
    Sizable,
    input::{Input, InputState},
};

/// Native editing with shared field geometry, typography and visual states.
/// Search/password variants compose this same Input using prefix/suffix builders.
/// Multiline InputState retains the requested editor height.
pub fn input(state: &Entity<InputState>, cx: &App) -> Input {
    Input::new(state).with_size(control_size(cx))
}

/// Select behavior with the shared field presentation, including popup search.
pub fn select<D: gpui_component::select::SelectDelegate + 'static>(
    state: &Entity<gpui_component::select::SelectState<D>>,
    cx: &App,
) -> gpui_component::select::Select<D> {
    gpui_component::select::Select::new(state).with_size(control_size(cx))
}

/// Calendar behavior with the same field trigger used by text and select controls.
pub fn date_picker(
    state: &Entity<gpui_component::date_picker::DatePickerState>,
    cx: &App,
) -> gpui_component::date_picker::DatePicker {
    gpui_component::date_picker::DatePicker::new(state).with_size(control_size(cx))
}

/// NumberInput composes the shared Input and matching square stepper buttons.
pub fn number_input(state: &Entity<InputState>, cx: &App) -> gpui_component::input::NumberInput {
    gpui_component::input::NumberInput::new(state).with_size(control_size(cx))
}

/// Native controls that can present an invalid state without filtering edits.
pub trait FieldControl: IntoElement + Sized {
    fn with_invalid(self, invalid: bool) -> Self;
}
impl FieldControl for Input {
    fn with_invalid(self, invalid: bool) -> Self {
        self.invalid(invalid)
    }
}
impl FieldControl for gpui_component::input::NumberInput {
    fn with_invalid(self, invalid: bool) -> Self {
        self.invalid(invalid)
    }
}
impl FieldControl for gpui_component::date_picker::DatePicker {
    fn with_invalid(self, invalid: bool) -> Self {
        self.invalid(invalid)
    }
}
impl<D: gpui_component::select::SelectDelegate + 'static> FieldControl
    for gpui_component::select::Select<D>
{
    fn with_invalid(self, invalid: bool) -> Self {
        self.invalid(invalid)
    }
}

/// Reusable labeled field. Error messages take precedence over descriptions and
/// apply the same invalid treatment to every supported native field control.
#[derive(IntoElement)]
pub struct RustyField<C: FieldControl + 'static> {
    label: SharedString,
    control: C,
    description: Option<SharedString>,
    error: Option<SharedString>,
    required: bool,
}
impl<C: FieldControl + 'static> RustyField<C> {
    pub fn new(label: impl Into<SharedString>, control: C) -> Self {
        Self {
            label: label.into(),
            control,
            description: None,
            error: None,
            required: false,
        }
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn error(mut self, error: Option<SharedString>) -> Self {
        self.error = error.filter(|message| !message.trim().is_empty());
        self
    }
    /// Marks the label as required; the app owns validation policy.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}
impl<C: FieldControl + 'static> RenderOnce for RustyField<C> {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let invalid = self.error.is_some();
        let label = if self.required {
            format!("{} *", self.label).into()
        } else {
            self.label
        };
        crate::field(label, self.control.with_invalid(invalid), cx).when_some(
            self.error.or(self.description),
            |this, message| {
                this.child(
                    text(message, TextStyle::Caption, cx)
                        .when(invalid, |this| this.text_color(color(tokens(cx).danger))),
                )
            },
        )
    }
}
