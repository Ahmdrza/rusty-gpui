//! A complete app with text, a button and a notification.
use rusty_gpui::gpui::{prelude::*, *};
use rusty_gpui::prelude::TextStyle;
use rusty_gpui::prelude::*;

struct Hello;

impl Render for Hello {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        stack(cx)
            .p_6()
            .child(text("Hello, Rusty GPUI", TextStyle::Heading, cx))
            .child(
                button("hello", "Say hello", ActionStyle::Primary, cx)
                    .on_click(|_, window, cx| window.push_notification("Hello!", cx)),
            )
    }
}

fn main() {
    rusty_gpui::run("Hello", |_, cx| cx.new(|_| Hello));
}
