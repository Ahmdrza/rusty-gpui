//! Single-window application setup for small apps and examples.
use gpui::{prelude::*, *};
use gpui_component::Root;

/// Launch a single-window Rusty GPUI application.
///
/// Installs the bundled assets, fonts and default theme, wraps the returned view
/// in [`Root`], and renders dialogs, sheets and notifications automatically.
/// The initial window is centered at 720×520 logical pixels, reduced to fit the
/// primary display. The application quits when its last window closes. Call
/// this from `main`.
///
/// Your view renders its content; the launcher owns the overlay layers. For
/// custom assets, multiple initial windows or custom window options, use GPUI's
/// [`Application`] directly (see the `minimal` example).
///
/// # Panics
/// Panics if the platform cannot open the application window.
pub fn run<V: Render + 'static>(
    title: impl Into<SharedString>,
    build: impl FnOnce(&mut Window, &mut App) -> Entity<V> + 'static,
) {
    let title = title.into();
    Application::new()
        .with_assets(crate::Assets)
        .run(move |cx| {
            crate::init(cx);
            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.defer(|cx| cx.quit());
                }
            })
            .detach();
            let display = cx
                .primary_display()
                .map(|display| display.bounds().size)
                .unwrap_or(size(px(720.), px(560.)));
            let bounds = Bounds::centered(
                None,
                size(
                    px(720.).min(display.width),
                    px(520.).min((display.height - px(40.)).max(px(1.))),
                ),
                cx,
            );
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(title),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                move |window, cx| {
                    let view = build(window, cx);
                    let content = cx.new(|_| AppContent { view });
                    cx.new(|cx| Root::new(content, window, cx))
                },
            )
            .expect("Unable to open the Rusty GPUI application window");
            cx.activate(true);
        });
}

struct AppContent<V: Render> {
    view: Entity<V>,
}

impl<V: Render + 'static> Render for AppContent<V> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheets = Root::render_sheet_layer(window, cx);
        let dialogs = Root::render_dialog_layer(window, cx);
        let notifications = Root::render_notification_layer(window, cx);
        let theme = crate::tokens(cx);
        div()
            .relative()
            .size_full()
            .bg(crate::color(theme.background))
            .text_color(crate::color(theme.foreground))
            .font_family(theme.font_family.clone())
            .text_size(px(theme.font_size))
            .child(self.view.clone())
            .children(sheets)
            .children(dialogs)
            .children(notifications)
    }
}
