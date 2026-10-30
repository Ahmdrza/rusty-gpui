//! A complete local settings workflow; business state and storage belong to the app.
mod store;

use rusty_gpui::components::dialog::DialogButtonProps;
use rusty_gpui::gpui::{prelude::*, *};
use rusty_gpui::prelude::*;
use rusty_gpui::prelude::{InputEvent, TextStyle};
use rusty_gpui::theme::{Mode, ThemeConfig};
use store::{Settings, Store};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    General,
    Appearance,
}

struct SettingsApp {
    name: Entity<InputState>,
    updates: bool,
    dark_mode: bool,
    saved: Option<Settings>,
    page: Page,
    loading: bool,
    saving: bool,
    load_failed: bool,
    error: Option<String>,
    store: Store,
    _changes: Subscription,
    task: Option<Task<()>>,
}

impl SettingsApp {
    fn new(store: Store, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx));
        let changes = cx.subscribe(&name, |_, _, _: &InputEvent, cx| cx.notify());
        let view = cx.entity().downgrade();
        window.on_window_should_close(cx, move |window, cx| {
            view.update(cx, |this, cx| {
                if this.saving {
                    window.push_notification("Wait for saving to finish.", cx);
                    return false;
                }
                if this.loading || this.load_failed || !this.dirty(cx) {
                    return true;
                }
                if window.has_active_dialog(cx) {
                    return false;
                }
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .title("Discard unsaved changes?")
                        .child("Your changes have not been saved.")
                        .confirm()
                        .button_props(
                            DialogButtonProps::default()
                                .ok_text("Discard")
                                .cancel_text("Keep editing"),
                        )
                        .on_ok(|_, window, cx| {
                            window.defer(cx, |window, _| window.remove_window());
                            true
                        })
                });
                false
            })
            .unwrap_or(true)
        });
        let mut app = Self {
            name,
            updates: false,
            dark_mode: false,
            saved: None,
            page: Page::General,
            loading: false,
            saving: false,
            load_failed: false,
            error: None,
            store,
            _changes: changes,
            task: None,
        };
        app.load(window, cx);
        app
    }

    fn current(&self, cx: &App) -> Settings {
        Settings {
            workspace: self.name.read(cx).value().to_string(),
            updates: self.updates,
            dark_mode: self.dark_mode,
        }
    }

    fn dirty(&self, cx: &App) -> bool {
        self.saved.as_ref() != Some(&self.current(cx))
    }

    fn apply(&mut self, settings: &Settings, window: &mut Window, cx: &mut Context<Self>) {
        self.name.update(cx, |name, cx| {
            name.set_value(settings.workspace.clone(), window, cx)
        });
        self.updates = settings.updates;
        self.dark_mode = settings.dark_mode;
        self.apply_theme(window, cx);
    }

    fn apply_theme(&self, window: &mut Window, cx: &mut App) {
        rusty_gpui::apply_theme(
            ThemeConfig {
                mode: if self.dark_mode {
                    Mode::Dark
                } else {
                    Mode::Light
                },
                ..Default::default()
            },
            Some(window),
            cx,
        );
    }

    fn load(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.loading || self.saving {
            return;
        }
        self.loading = true;
        self.error = None;
        let store = self.store.clone();
        let read = cx.background_executor().spawn(async move { store.load() });
        self.task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = read.await;
            let _ = this.update_in(cx, |this, window, cx| {
                this.loading = false;
                match result {
                    Ok(settings) => {
                        this.apply(&settings.clone().unwrap_or_default(), window, cx);
                        this.saved = settings;
                        this.load_failed = false;
                    }
                    Err(error) => {
                        this.error = Some(error);
                        this.load_failed = true;
                    }
                }
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.loading || self.saving || self.load_failed {
            return;
        }
        let settings = self.current(cx);
        if let Err(error) = settings.validate() {
            self.error = Some(error.into());
            cx.notify();
            return;
        }
        self.saving = true;
        self.error = None;
        let store = self.store.clone();
        let snapshot = settings.clone();
        let write = cx
            .background_executor()
            .spawn(async move { store.save(&snapshot) });
        self.task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = write.await;
            let _ = this.update_in(cx, |this, window, cx| {
                this.saving = false;
                match result {
                    Ok(()) => {
                        this.saved = Some(settings);
                        window.push_notification("Settings saved", cx);
                    }
                    Err(error) => this.error = Some(error),
                }
                cx.notify();
            });
        }));
        cx.notify();
    }
}

impl Render for SettingsApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let busy = self.loading || self.saving;
        let disabled = busy || self.load_failed;
        let current = self.current(cx);
        let dirty = self.dirty(cx);
        let validation = (!disabled).then(|| current.validate().err()).flatten();
        div().id("settings-scroll").size_full().overflow_y_scroll()
            .child(stack(cx).p_6().w_full().max_w(px(640.)).mx_auto()
                .child(text("Workspace settings", TextStyle::Heading, cx))
                .child(text("A local application built with Rusty GPUI", TextStyle::Caption, cx))
                .child(inline(cx)
                    .child(button("general", "General", if self.page == Page::General { ActionStyle::Primary } else { ActionStyle::Secondary }, cx)
                        .on_click(cx.listener(|this, _, _, cx| { this.page = Page::General; cx.notify(); })))
                    .child(button("appearance", "Appearance", if self.page == Page::Appearance { ActionStyle::Primary } else { ActionStyle::Secondary }, cx)
                        .on_click(cx.listener(|this, _, _, cx| { this.page = Page::Appearance; cx.notify(); }))))
                .when(self.loading, |view| view.child(text("Loading settings…", TextStyle::Body, cx)))
                .when(self.page == Page::General, |view| view
                    .child(RustyField::new("Workspace name", input(&self.name, cx).disabled(disabled))
                        .required(true).description("Shown in your workspace. Up to 80 characters.")
                        .error(validation.map(Into::into)))
                    .child(Checkbox::new("updates").label("Receive product updates").checked(self.updates).disabled(disabled)
                        .on_click(cx.listener(|this, checked, _, cx| { this.updates = *checked; cx.notify(); }))))
                .when(self.page == Page::Appearance, |view| view
                    .child(Switch::new("dark-mode").label("Dark appearance").checked(self.dark_mode).disabled(disabled)
                        .on_click(cx.listener(|this, checked, window, cx| { this.dark_mode = *checked; this.apply_theme(window, cx); cx.notify(); })))
                    .child(text("Appearance previews immediately. Save to keep it after restarting.", TextStyle::Body, cx)))
                .when_some(self.error.clone(), |view, error| view.child(text(error, TextStyle::Body, cx)))
                .when(self.load_failed && !busy, |view| view.child(inline(cx)
                    .child(button("retry", "Retry loading", ActionStyle::Secondary, cx)
                        .on_click(cx.listener(|this, _, window, cx| this.load(window, cx))))
                    .child(button("defaults", "Use defaults", ActionStyle::Secondary, cx)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.apply(&Settings::default(), window, cx);
                            this.load_failed = false; this.error = None; cx.notify();
                        })))))
                .child(text(if self.load_failed { "The existing file is unchanged. Use defaults to replace it on your next save." }
                    else if busy { "Please wait…" }
                    else if dirty { "Unsaved changes" }
                    else { "All changes saved" }, TextStyle::Caption, cx))
                .child(inline(cx)
                    .child(button("save", "Save changes", ActionStyle::Primary, cx).loading(self.saving)
                        .disabled(disabled || validation.is_some() || !dirty)
                        .on_click(cx.listener(|this, _, window, cx| this.save(window, cx))))
                    .child(button("revert", "Revert", ActionStyle::Secondary, cx).disabled(disabled || !dirty)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.apply(&this.saved.clone().unwrap_or_default(), window, cx);
                            this.error = None; cx.notify();
                        }))))
            )
    }
}

fn main() {
    let path = store::settings_path().unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1)
    });
    rusty_gpui::run("Workspace settings", move |window, cx| {
        cx.new(|cx| SettingsApp::new(Store { path }, window, cx))
    });
}
