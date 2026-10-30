//! Dialog, drawer and notification layers are supplied by rusty_gpui::run.
use rusty_gpui::gpui::{prelude::*, *};
use rusty_gpui::prelude::TextStyle;
use rusty_gpui::prelude::*;

struct Overlays;

impl Render for Overlays {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        stack(cx)
            .p_6()
            .child(text("Overlay examples", TextStyle::Heading, cx))
            .child(
                button("dialog", "Open dialog", ActionStyle::Primary, cx).on_click(
                    |_, window, cx| {
                        window.open_dialog(cx, |dialog, _, _| {
                            dialog
                                .title("Ready to continue?")
                                .child("This is a local demonstration.")
                                .confirm()
                                .on_ok(|_, window, cx| {
                                    window.push_notification("Confirmed", cx);
                                    true
                                })
                        });
                    },
                ),
            )
            .child(
                button("drawer", "Open drawer", ActionStyle::Secondary, cx).on_click(
                    |_, window, cx| {
                        window.open_sheet(cx, |sheet, _, cx| {
                            sheet.title("Details").child(
                                button("done", "Done", ActionStyle::Primary, cx)
                                    .on_click(|_, window, cx| window.close_sheet(cx)),
                            )
                        });
                    },
                ),
            )
    }
}

fn main() {
    rusty_gpui::run("Overlays", |_, cx| cx.new(|_| Overlays));
}
