//! A small independent app using Rusty GPUI; no gallery or separate assets dependency.
use gpui::{prelude::*, *};
use rusty_gpui::prelude::*;
use rusty_gpui::prelude::{InputEvent, TextStyle};

struct Example {
    value: Entity<InputState>,
    enabled: bool,
    _changes: Subscription,
}
impl Example {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let value = cx.new(|cx| InputState::new(window, cx).default_value("Hello"));
        let changes = cx.subscribe(&value, |_, _, _: &InputEvent, cx| cx.notify());
        Self {
            value,
            enabled: true,
            _changes: changes,
        }
    }
}
impl Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let empty = self.value.read(cx).value().trim().is_empty();
        let sheet = Root::render_sheet_layer(window, cx);
        let dialog = Root::render_dialog_layer(window, cx);
        let notifications = Root::render_notification_layer(window, cx);
        div()
            .relative()
            .size_full()
            .bg(color(tokens(cx).background))
            .text_color(color(tokens(cx).foreground))
            .child(
                stack(cx)
                    .p(px(tokens(cx).spacing[6]))
                    .child(text("Built with Rusty GPUI", TextStyle::Heading, cx))
                    .child(
                        RustyField::new("Value", input(&self.value, cx).disabled(!self.enabled))
                            .required(true)
                            .description("Try clearing this field to see its error state.")
                            .error((self.enabled && empty).then(|| "Enter a value.".into())),
                    )
                    .child(
                        Switch::new("enabled")
                            .label("Enable field")
                            .checked(self.enabled)
                            .on_click(cx.listener(|this, enabled, _, cx| {
                                this.enabled = *enabled;
                                cx.notify();
                            })),
                    )
                    .child(
                        button("save", "Save", ActionStyle::Primary, cx)
                            .disabled(!self.enabled || empty)
                            .on_click(|_, window, cx| window.push_notification("Saved", cx)),
                    ),
            )
            .children(sheet)
            .children(dialog)
            .children(notifications)
    }
}
fn main() {
    Application::new()
        .with_assets(rusty_gpui::Assets)
        .run(|cx| {
            rusty_gpui::init(cx);
            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.defer(|cx| cx.quit());
                }
            })
            .detach();
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(560.), px(400.)),
                        cx,
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Rusty GPUI starter".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| Example::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("Unable to open window");
            cx.activate(true);
        });
}
