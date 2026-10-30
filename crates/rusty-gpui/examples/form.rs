//! Editable state, field validation, a checkbox, a switch and a save action.
use rusty_gpui::gpui::{prelude::*, *};
use rusty_gpui::prelude::*;
use rusty_gpui::prelude::{InputEvent, TextStyle};

struct Form {
    name: Entity<InputState>,
    enabled: bool,
    updates: bool,
    _changes: Subscription,
}

impl Form {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx).default_value("My workspace"));
        // Keep the subscription alive so the parent redraws its validation/button.
        let changes = cx.subscribe(&name, |_, _, _: &InputEvent, cx| cx.notify());
        Self {
            name,
            enabled: true,
            updates: false,
            _changes: changes,
        }
    }
}

impl Render for Form {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let empty = self.name.read(cx).value().trim().is_empty();
        stack(cx)
            .p_6()
            .child(text("Workspace settings", TextStyle::Heading, cx))
            .child(
                RustyField::new("Name", input(&self.name, cx).disabled(!self.enabled))
                    .required(true)
                    .error((self.enabled && empty).then(|| "Enter a name.".into())),
            )
            .child(
                Checkbox::new("updates")
                    .label("Include product updates")
                    .checked(self.updates)
                    .on_click(cx.listener(|this, checked, _, cx| {
                        this.updates = *checked;
                        cx.notify();
                    })),
            )
            .child(
                Switch::new("editing")
                    .label("Enable editing")
                    .checked(self.enabled)
                    .on_click(cx.listener(|this, enabled, _, cx| {
                        this.enabled = *enabled;
                        cx.notify();
                    })),
            )
            .child(
                button("save", "Save", ActionStyle::Primary, cx)
                    .disabled(!self.enabled || empty)
                    .on_click(cx.listener(|this, _, window, cx| {
                        let name = this.name.read(cx).value();
                        let updates = if this.updates { "on" } else { "off" };
                        window.push_notification(format!("Saved {name}; updates {updates}"), cx);
                    })),
            )
    }
}

fn main() {
    rusty_gpui::run("Workspace settings", |window, cx| {
        cx.new(|cx| Form::new(window, cx))
    });
}
