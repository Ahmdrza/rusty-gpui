use gpui::{prelude::*, *};
use gpui_component::table::{Column, ColumnSort, Table, TableDelegate, TableState};
use rusty_gpui::prelude::*;
use rusty_gpui::prelude::{InputEvent, TextStyle};
use std::time::Instant;
mod settings;

const CATEGORIES: [&str; 10] = [
    "All components",
    "Foundations",
    "Actions",
    "Inputs",
    "Selection",
    "Navigation",
    "Overlays",
    "Feedback",
    "Data",
    "Performance lab",
];

const PROJECT_NAMES: [&str; 5] = ["Atlas", "Beacon", "Catalyst", "Daybreak", "Evergreen"];

fn project_matches(query: &str) -> Vec<&'static str> {
    let query = query.trim().to_lowercase();
    PROJECT_NAMES
        .into_iter()
        .filter(|name| name.to_lowercase().contains(&query))
        .collect()
}

fn collection_matches(category: usize, current: usize, keywords: &str, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        current == 0 || current == category
    } else {
        query.split_whitespace().all(|term| keywords.contains(term))
    }
}

struct ProjectTable {
    columns: Vec<Column>,
    rows: usize,
    descending: bool,
    order: Vec<usize>,
}
impl ProjectTable {
    fn new() -> Self {
        let mut table = Self {
            columns: vec![
                Column::new("name", "Name").width(px(280.)).ascending(),
                Column::new("status", "Status").width(px(170.)),
                Column::new("owner", "Owner").width(px(180.)),
            ],
            rows: 100,
            descending: false,
            order: Vec::new(),
        };
        table.set_rows(100);
        table
    }
    fn set_rows(&mut self, rows: usize) {
        self.rows = rows;
        self.order = (0..rows).collect();
        self.order.sort_by_key(|row| (row % 5, *row));
        if self.descending {
            self.order.reverse();
        }
    }
    fn reorder_column(&mut self, from: usize, to: usize) {
        let column = self.columns.remove(from);
        self.columns.insert(to, column);
    }
    fn cell_value(&self, row: usize, col: usize) -> String {
        let row = self.order[row];
        match self.columns[col].key.as_ref() {
            "name" => format!("{} / {:05}", PROJECT_NAMES[row % 5], row + 1),
            "status" => ["In progress", "Complete", "Draft"][row % 3].to_owned(),
            _ => ["Alex Chen", "Daniel Park", "Taylor Kim"][row % 3].to_owned(),
        }
    }
}
impl TableDelegate for ProjectTable {
    fn columns_count(&self, _: &App) -> usize {
        self.columns.len()
    }
    fn rows_count(&self, _: &App) -> usize {
        self.rows
    }
    fn column(&self, index: usize, _: &App) -> &Column {
        &self.columns[index]
    }
    fn perform_sort(
        &mut self,
        _: usize,
        sort: ColumnSort,
        _: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) {
        self.descending = sort == ColumnSort::Descending;
        self.set_rows(self.rows);
        cx.notify();
    }
    fn render_td(
        &mut self,
        row: usize,
        col: usize,
        _: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        text(self.cell_value(row, col), TextStyle::Body, cx)
    }
    fn move_column(
        &mut self,
        from: usize,
        to: usize,
        _: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) {
        self.reorder_column(from, to);
        cx.notify();
    }
}

struct Gallery {
    config: ThemeConfig,
    category: usize,
    search: Entity<InputState>,
    demo_search: Entity<InputState>,
    name: Entity<InputState>,
    disabled_input: Entity<InputState>,
    email: Entity<InputState>,
    password: Entity<InputState>,
    description: Entity<InputState>,
    quantity: Entity<InputState>,
    custom_primary: Entity<InputState>,
    custom_secondary: Entity<InputState>,
    modal_name: Entity<InputState>,
    select: Entity<SelectState<SearchableVec<SharedString>>>,
    date: Entity<DatePickerState>,
    slider: Entity<SliderState>,
    table: Entity<TableState<ProjectTable>>,
    page_scroll: ScrollHandle,
    checked: bool,
    switched: bool,
    radio: usize,
    tab: usize,
    segment: usize,
    page: usize,
    clicks: usize,
    loading: bool,
    compact_theme_visible: bool,
    fields_disabled: bool,
    disable_fields_task: Option<Task<()>>,
    progress: f32,
    events: Vec<String>,
    subscriptions: Vec<Subscription>,
    benchmark_remaining: usize,
    benchmark_last: Option<Instant>,
    frame_samples: Vec<f64>,
    benchmark_result: String,
}

impl Gallery {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let make_input =
            |placeholder: &'static str, window: &mut Window, cx: &mut Context<Self>| {
                cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
            };
        let search = make_input("Find a component…", window, cx);
        let demo_search = make_input("Search projects…", window, cx);
        let name = make_input("Acme Studio", window, cx);
        let disabled_input = make_input("Not available", window, cx);
        let email = make_input("you@example.com", window, cx);
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter password")
                .masked(true)
        });
        let description = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("What are you working on?")
                .multi_line(true)
                .rows(3)
        });
        let quantity = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("1")
                .validate(|value, _| value.is_empty() || value.parse::<u32>().is_ok())
        });
        let custom_primary = make_input("#4567D5", window, cx);
        let custom_secondary = make_input("#C36A32", window, cx);
        let modal_name = make_input("Workspace name", window, cx);
        let select = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(vec![
                    "All projects".into(),
                    "Assigned to me".into(),
                    "Recently updated".into(),
                    "Archived".into(),
                ]),
                Some(IndexPath::new(0)),
                window,
                cx,
            )
            .searchable(true)
        });
        let date = cx.new(|cx| DatePickerState::new(window, cx));
        let slider = cx.new(|_| {
            SliderState::new()
                .min(0.)
                .max(100.)
                .step(1.)
                .default_value(64.)
        });
        let table = cx.new(|cx| {
            TableState::new(ProjectTable::new(), window, cx)
                .sortable(true)
                .col_resizable(true)
        });
        let mut this = Self {
            config: settings::load(),
            category: 0,
            search,
            demo_search,
            name,
            disabled_input,
            email,
            password,
            description,
            quantity,
            custom_primary,
            custom_secondary,
            modal_name,
            select,
            date,
            slider,
            table,
            page_scroll: ScrollHandle::new(),
            checked: true,
            switched: true,
            radio: 0,
            tab: 0,
            segment: 0,
            page: 0,
            clicks: 0,
            loading: false,
            compact_theme_visible: false,
            fields_disabled: false,
            disable_fields_task: None,
            progress: 64.,
            events: Vec::new(),
            subscriptions: Vec::new(),
            benchmark_remaining: 0,
            benchmark_last: None,
            frame_samples: Vec::new(),
            benchmark_result: "Not run yet".into(),
        };
        this.subscriptions
            .push(cx.subscribe(&this.search, |this, _, _: &InputEvent, cx| {
                this.page_scroll.set_offset(point(px(0.), px(0.)));
                cx.notify();
            }));
        this.subscriptions
            .push(cx.subscribe(&this.demo_search, |_, _, _: &InputEvent, cx| cx.notify()));
        this.subscriptions
            .push(cx.subscribe(&this.email, |_, _, _: &InputEvent, cx| cx.notify()));
        this.subscriptions.push(
            cx.subscribe(&this.slider, |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                this.progress = value.start();
                this.log(format!("Slider: {:.0}", this.progress), cx);
            }),
        );
        this.subscriptions.push(cx.subscribe(
            &this.select,
            |this, _, _: &SelectEvent<SearchableVec<SharedString>>, cx| {
                this.log("Select changed", cx)
            },
        ));
        this.subscriptions.push(
            cx.subscribe(&this.date, |this, _, _: &DatePickerEvent, cx| {
                this.log("Date changed", cx)
            }),
        );
        this.subscriptions.push(cx.subscribe_in(
            &this.quantity,
            window,
            |this, _, event: &gpui_component::input::NumberInputEvent, window, cx| {
                let gpui_component::input::NumberInputEvent::Step(step) = event;
                let current = this.quantity.read(cx).value().parse::<u32>().unwrap_or(0);
                let next = match step {
                    gpui_component::input::StepAction::Increment => current.saturating_add(1),
                    gpui_component::input::StepAction::Decrement => current.saturating_sub(1),
                };
                this.quantity.update(cx, |state, cx| {
                    state.set_value(next.to_string(), window, cx)
                });
                cx.notify();
            },
        ));
        this.sync_color_fields(window, cx);
        #[cfg(debug_assertions)]
        if std::env::var_os("RUSTY_GPUI_SMOKE_STATE").is_some() {
            this.subscriptions
                .push(cx.observe(&this.table, |_, _, cx| cx.notify()));
        }
        this
    }
    fn log(&mut self, message: impl Into<String>, cx: &mut Context<Self>) {
        self.events.insert(0, message.into());
        self.events.truncate(12);
        cx.notify();
    }
    fn apply(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.sync_color_fields(window, cx);
        apply_theme(self.config.clone(), Some(window), cx);
        if let Err(error) = settings::save(&self.config) {
            self.log(format!("Could not save theme: {error}"), cx);
        }
        self.log("Theme updated", cx);
    }
    fn sync_color_fields(&self, window: &mut Window, cx: &mut Context<Self>) {
        let t = self.config.generate();
        self.custom_primary.update(cx, |state, cx| {
            state.set_value(t.primary.to_hex(), window, cx)
        });
        self.custom_secondary.update(cx, |state, cx| {
            state.set_value(t.secondary.to_hex(), window, cx)
        });
    }
    fn visible(&self, category: usize, keywords: &str, cx: &App) -> bool {
        let query = self.search.read(cx).value().to_string().to_lowercase();
        collection_matches(category, self.category, keywords, &query)
    }
    fn row(&self, cx: &App) -> Div {
        inline(cx)
    }
    fn column(&self, cx: &App) -> Div {
        stack(cx)
    }
    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx).clone();
        v_flex()
            .w(px(216.))
            .h_full()
            .flex_shrink_0()
            .p(px(20.))
            .gap(px(8.))
            .border_r_1()
            .border_color(color(t.border))
            .bg(color(t.surface))
            .child(
                h_flex()
                    .gap_3()
                    .pb_6()
                    .child(
                        div()
                            .size(px(26.))
                            .bg(color(t.foreground))
                            .rounded(px(5.))
                            .text_color(color(t.background))
                            .flex()
                            .justify_center()
                            .items_center()
                            .child("R"),
                    )
                    .child(text("Rusty GPUI", TextStyle::Subheading, cx)),
            )
            .child(text("COMPONENT LIBRARY", TextStyle::Caption, cx))
            .children(CATEGORIES.iter().enumerate().map(|(index, label)| {
                button(("category", index), *label, ActionStyle::Quiet, cx)
                    .w_full()
                    .justify_start()
                    .when(self.category == index, |b| {
                        b.bg(color(t.primary_subtle))
                            .text_color(color(t.primary_text))
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.category = index;
                        this.search
                            .update(cx, |state, cx| state.set_value("", window, cx));
                        this.page_scroll.set_offset(point(px(0.), px(0.)));
                        cx.notify();
                    }))
            }))
            .child(div().flex_1())
            .child(text("Native / Rust + GPUI", TextStyle::Caption, cx))
            .child(text(
                "Rusty GPUI 0.4.0 · Development",
                TextStyle::Caption,
                cx,
            ))
    }
    fn theme_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx).clone();
        v_flex()
            .w(px(238.))
            .flex_shrink_0()
            .border_l_1()
            .border_color(color(t.border))
            .bg(color(t.surface))
            .p(px(20.))
            .gap(px(18.))
            .child(text("Make it yours", TextStyle::Subheading, cx))
            .child(text(
                "Every control uses the same tokens.",
                TextStyle::Caption,
                cx,
            ))
            .child(field(
                "Appearance",
                self.row(cx)
                    .children([("Light", Mode::Light), ("Dark", Mode::Dark)].map(
                        |(label, mode)| {
                            button(
                                SharedString::from(format!("mode-{label}")),
                                label,
                                if self.config.mode == mode {
                                    ActionStyle::Primary
                                } else {
                                    ActionStyle::Secondary
                                },
                                cx,
                            )
                            .on_click(cx.listener(
                                move |this, _, window, cx| {
                                    this.config.mode = mode;
                                    this.apply(window, cx);
                                },
                            ))
                        },
                    )),
                cx,
            ))
            .child(field(
                "Accent",
                self.row(cx).children(
                    ["#4567D5", "#247A58", "#C36A32", "#8A58C7", "#666666"]
                        .iter()
                        .enumerate()
                        .map(|(i, hex)| {
                            let c = Color::hex(hex).unwrap();
                            button(("accent", i), " ", ActionStyle::Quiet, cx)
                                .bg(color(c))
                                .w(px(28.))
                                .h(px(28.))
                                .tooltip(*hex)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.config.primary = c;
                                    this.apply(window, cx);
                                }))
                        }),
                ),
                cx,
            ))
            .child(input(&self.custom_primary, cx))
            .child(
                button("apply-hex", "Apply hex color", ActionStyle::Secondary, cx).on_click(
                    cx.listener(|this, _, window, cx| {
                        match Color::hex(&this.custom_primary.read(cx).value()) {
                            Ok(c) => {
                                this.config.primary = c;
                                this.apply(window, cx);
                            }
                            Err(e) => window.push_notification(
                                (NotificationType::Error, SharedString::from(e.to_string())),
                                cx,
                            ),
                        }
                    }),
                ),
            )
            .child(field(
                "Secondary color",
                input(&self.custom_secondary, cx),
                cx,
            ))
            .child(
                self.row(cx)
                    .child(
                        button("apply-secondary", "Use hex", ActionStyle::Secondary, cx).on_click(
                            cx.listener(|this, _, window, cx| {
                                match Color::hex(&this.custom_secondary.read(cx).value()) {
                                    Ok(c) => {
                                        this.config.secondary = Some(c);
                                        this.apply(window, cx);
                                    }
                                    Err(e) => window.push_notification(
                                        (
                                            NotificationType::Error,
                                            SharedString::from(e.to_string()),
                                        ),
                                        cx,
                                    ),
                                }
                            }),
                        ),
                    )
                    .child(
                        button("auto-secondary", "Auto", ActionStyle::Quiet, cx).on_click(
                            cx.listener(|this, _, window, cx| {
                                this.config.secondary = None;
                                this.apply(window, cx);
                            }),
                        ),
                    ),
            )
            .child(field(
                "Density",
                v_flex().gap_1().children(
                    [
                        ("Compact", Density::Compact),
                        ("Comfortable", Density::Comfortable),
                        ("Spacious", Density::Spacious),
                    ]
                    .map(|(label, density)| {
                        button(
                            SharedString::from(format!("density-{label}")),
                            label,
                            if self.config.density == density {
                                ActionStyle::Primary
                            } else {
                                ActionStyle::Quiet
                            },
                            cx,
                        )
                        .on_click(cx.listener(
                            move |this, _, window, cx| {
                                this.config.density = density;
                                this.apply(window, cx);
                            },
                        ))
                    }),
                ),
                cx,
            ))
            .child(field(
                "Font size",
                self.row(cx)
                    .child(
                        button("font-minus", "−", ActionStyle::Secondary, cx).on_click(
                            cx.listener(|this, _, window, cx| {
                                this.config.font_size = (this.config.font_size - 1.).max(11.);
                                this.apply(window, cx);
                            }),
                        ),
                    )
                    .child(text(
                        format!("{:.0} px", self.config.font_size),
                        TextStyle::Body,
                        cx,
                    ))
                    .child(
                        button("font-plus", "+", ActionStyle::Secondary, cx).on_click(cx.listener(
                            |this, _, window, cx| {
                                this.config.font_size = (this.config.font_size + 1.).min(22.);
                                this.apply(window, cx);
                            },
                        )),
                    ),
                cx,
            ))
            .child(field(
                "Font family",
                self.row(cx)
                    .children(
                        ["Inter", ".SystemUIFont", "Helvetica Neue", "Menlo"].map(|font| {
                            button(
                                SharedString::from(format!("font-{font}")),
                                font,
                                ActionStyle::Quiet,
                                cx,
                            )
                            .on_click(cx.listener(
                                move |this, _, window, cx| {
                                    this.config.font_family = font.into();
                                    this.apply(window, cx);
                                },
                            ))
                        }),
                    ),
                cx,
            ))
            .child(field(
                "Corner radius",
                self.row(cx).children([0_u32, 5, 10].map(|radius| {
                    button(
                        ("radius", radius),
                        format!("{radius}"),
                        ActionStyle::Secondary,
                        cx,
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.config.radius = radius as f32;
                        this.apply(window, cx);
                    }))
                })),
                cx,
            ))
            .child(field(
                "Spacing unit",
                self.row(cx).children([3_u32, 4, 6].map(|unit| {
                    button(
                        ("unit", unit),
                        format!("{unit} px"),
                        ActionStyle::Secondary,
                        cx,
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.config.spacing_unit = unit as f32;
                        this.apply(window, cx);
                    }))
                })),
                cx,
            ))
            .child(
                button(
                    "settings-json",
                    "Reveal theme JSON",
                    ActionStyle::Secondary,
                    cx,
                )
                .on_click(cx.listener(|this, _, window, cx| match settings::save(
                    &this.config,
                ) {
                    Ok(()) => cx.reveal_path(&settings::path()),
                    Err(error) => window.push_notification(
                        (NotificationType::Error, SharedString::from(error)),
                        cx,
                    ),
                })),
            )
            .child(
                button("reset", "Reset theme", ActionStyle::Quiet, cx).on_click(cx.listener(
                    |this, _, window, cx| {
                        this.config = ThemeConfig::default();
                        this.apply(window, cx);
                    },
                )),
            )
    }
    fn foundations(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx).clone();
        section(
            "01",
            "Foundations",
            "Type, space and color. Configured once, applied everywhere.",
            self.column(cx)
                .child(text("Primitives, precisely.", TextStyle::Display, cx))
                .child(rusty_gpui::surface(SurfaceRole::Floating, cx)
                    .p(px(t.spacing[4]))
                    .child(stack(cx)
                        .child(text("Shared component foundations", TextStyle::Label, cx))
                        .child(text(format!("Control height {:.0}px · Base type {:.0}px · Radius {:.0}px", t.control_height, t.font_size, t.radius), TextStyle::Caption, cx))
                        .child(text("Actions · Fields · Selection · Navigation · Surfaces · Feedback · Data rows", TextStyle::Body, cx))))
                .child(text(
                    "A foundation for whatever comes next.",
                    TextStyle::Heading,
                    cx,
                ))
                .child(text("Everything in its place", TextStyle::Subheading, cx))
                .child(text(
                    "Body text gives your interface a clear, comfortable rhythm.",
                    TextStyle::Body,
                    cx,
                ))
                .child(text(
                    "Caption / supporting information",
                    TextStyle::Caption,
                    cx,
                ))
                .child(
                    self.row(cx)
                        .children(t.scale.iter().enumerate().map(|(i, c)| {
                            v_flex()
                                .gap_1()
                                .child(
                                    div()
                                        .w(px(46.))
                                        .h(px(36.))
                                        .rounded(px(t.radius))
                                        .bg(color(*c)),
                                )
                                .child(text(format!("{}", i * 100), TextStyle::Caption, cx))
                        })),
                )
                .child(text(
                    format!(
                        "On-primary contrast: {:.2}:1 · Accent text: {:.2}:1",
                        t.primary.contrast(t.on_primary),
                        t.primary_text.contrast(t.background)
                    ),
                    TextStyle::Caption,
                    cx,
                ))
                .child(self.row(cx).children(t.spacing[1..7].iter().map(|s| {
                    v_flex()
                        .gap_2()
                        .child(div().w(px(*s)).h(px(8.)).bg(color(t.primary)))
                        .child(text(format!("{s:.0}"), TextStyle::Caption, cx))
                }))),
            cx,
        )
    }
    fn actions(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section(
            "02",
            "Actions",
            "Neutral primary actions. Accent and destructive variants when they carry meaning.",
            self.column(cx)
                .child(
                    self.row(cx).children(
                        [
                            ("Create workspace", ActionStyle::Primary),
                            ("View details", ActionStyle::Secondary),
                            ("Cancel", ActionStyle::Quiet),
                            ("Accent", ActionStyle::Accent),
                            ("Secondary accent", ActionStyle::SecondaryAccent),
                            ("Delete", ActionStyle::Danger),
                        ]
                        .into_iter()
                        .enumerate()
                        .map(|(i, (label, style))| {
                            button(("action", i), label, style, cx).on_click(cx.listener(
                                move |this, _, _, cx| {
                                    this.clicks += 1;
                                    this.log(format!("{label} · click {}", this.clicks), cx);
                                },
                            ))
                        }),
                    ),
                )
                .child(
                    self.row(cx)
                        .child(
                            button("icon-button", "", ActionStyle::Secondary, cx)
                                .icon(IconName::Settings)
                                .tooltip("Open settings")
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.log("Settings clicked", cx)),
                                ),
                        )
                        .child(
                            button("disabled", "Unavailable", ActionStyle::Secondary, cx)
                                .disabled(true),
                        )
                        .child(
                            button(
                                "loading",
                                if self.loading {
                                    "Working…"
                                } else {
                                    "Toggle loading"
                                },
                                ActionStyle::Secondary,
                                cx,
                            )
                            .loading(self.loading)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.loading = true;
                                this.log("Loading started", cx);
                            })),
                        )
                        .child(
                            button("stop-loading", "Reset loading", ActionStyle::Quiet, cx)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.loading = false;
                                    this.log("Loading finished", cx);
                                })),
                        ),
                )
                .child(
                    self.row(cx)
                        .child(Button::new("small").label("Small").small())
                        .child(Button::new("medium").label("Medium"))
                        .child(Button::new("large").label("Large").large()),
                )
                .child(text(
                    format!("Action count: {}", self.clicks),
                    TextStyle::Caption,
                    cx,
                )),
            cx,
        )
    }
    fn inputs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let results = project_matches(&self.demo_search.read(cx).value());
        let email = self.email.read(cx).value().to_string();
        let invalid = !email.is_empty()
            && (!email.contains('@') || !email.rsplit('@').next().unwrap_or("").contains('.'));
        section(
            "03",
            "Inputs",
            "Real native text editing, selection, clipboard and input-method support.",
            self.column(cx)
                .child(field("Workspace name", input(&self.name, cx), cx))
                .child(field(
                    "Disabled example",
                    input(&self.disabled_input, cx).disabled(true),
                    cx,
                ))
                .child(
                    RustyField::new("Email", input(&self.email, cx))
                        .required(true)
                        .description("Used for account notifications.")
                        .error(invalid.then(|| "Enter a valid email address.".into())),
                )
                .child(field(
                    "Password",
                    input(&self.password, cx).mask_toggle(),
                    cx,
                ))
                .child(field(
                    "Description",
                    input(&self.description, cx).h(px(100.)),
                    cx,
                ))
                .child(field(
                    "Search",
                    input(&self.demo_search, cx)
                        .prefix(Icon::new(IconName::Search))
                        .cleanable(true),
                    cx,
                ))
                .child(text(
                    format!("{} matching projects", results.len()),
                    TextStyle::Caption,
                    cx,
                ))
                .children(results.iter().map(|name| text(*name, TextStyle::Body, cx)))
                .when(results.is_empty(), |d| {
                    d.child(text(
                        "No projects found. Try Atlas or Beacon.",
                        TextStyle::Caption,
                        cx,
                    ))
                })
                .child(
                    self.row(cx)
                        .child(
                            button(
                                "enable-fields",
                                "Enable pickers",
                                ActionStyle::Secondary,
                                cx,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.disable_fields_task = None;
                                this.fields_disabled = false;
                                cx.notify();
                            })),
                        )
                        .child(
                            button(
                                "disable-fields",
                                "Disable pickers in 3 seconds",
                                ActionStyle::Secondary,
                                cx,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.disable_fields_task = Some(cx.spawn(async |this, cx| {
                                    cx.background_executor()
                                        .timer(std::time::Duration::from_secs(3))
                                        .await;
                                    let _ = this.update(cx, |this, cx| {
                                        this.fields_disabled = true;
                                        cx.notify();
                                    });
                                }));
                            })),
                        )
                        .child(text(
                            if self.fields_disabled {
                                "Pickers disabled"
                            } else {
                                "Pickers enabled"
                            },
                            TextStyle::Caption,
                            cx,
                        )),
                )
                .child(field(
                    "Searchable select",
                    rusty_gpui::select(&self.select, cx).disabled(self.fields_disabled),
                    cx,
                ))
                .child(field("Quantity", number_input(&self.quantity, cx), cx))
                .child(field(
                    "Date picker",
                    rusty_gpui::date_picker(&self.date, cx)
                        .disabled(self.fields_disabled)
                        .placeholder("Choose a date")
                        .cleanable(true),
                    cx,
                )),
            cx,
        )
    }
    fn selection(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section(
            "04",
            "Selection",
            "Controlled state with visible results and keyboard interaction.",
            self.column(cx)
                .child(
                    self.row(cx)
                        .child(
                            Checkbox::new("checkbox")
                                .label("Assign to me")
                                .checked(self.checked)
                                .on_click(cx.listener(|this, value, _, cx| {
                                    this.checked = *value;
                                    this.log(format!("Checkbox: {value}"), cx);
                                })),
                        )
                        .child(
                            Checkbox::new("disabled-checkbox")
                                .label("Disabled")
                                .disabled(true),
                        )
                        .child(
                            Switch::new("switch")
                                .label("Private workspace")
                                .checked(self.switched)
                                .on_click(cx.listener(|this, value, _, cx| {
                                    this.switched = *value;
                                    this.log(format!("Switch: {value}"), cx);
                                })),
                        ),
                )
                .child(
                    RadioGroup::horizontal("radio")
                        .selected_index(Some(self.radio))
                        .children([
                            Radio::new("personal").label("Personal"),
                            Radio::new("team").label("Team"),
                            Radio::new("enterprise").label("Enterprise"),
                        ])
                        .on_click(cx.listener(|this, index, _, cx| {
                            this.radio = *index;
                            this.log(format!("Radio option: {index}"), cx);
                        })),
                )
                .child(
                    TabBar::new("segmented")
                        .segmented()
                        .selected_index(self.segment)
                        .children([
                            Tab::new().label("List"),
                            Tab::new().label("Board"),
                            Tab::new().label("Timeline"),
                        ])
                        .on_click(cx.listener(|this, index, _, cx| {
                            this.segment = *index;
                            this.log(format!("View: {index}"), cx);
                        })),
                )
                .child(field(
                    format!("Slider / {:.0}%", self.progress),
                    Slider::new(&self.slider).horizontal(),
                    cx,
                ))
                .child(Progress::new().value(self.progress)),
            cx,
        )
    }
    fn navigation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section(
            "05",
            "Navigation",
            "Tabs, breadcrumbs, pagination and menus.",
            self.column(cx)
                .child(
                    TabBar::new("tabs")
                        .underline()
                        .selected_index(self.tab)
                        .children([
                            Tab::new().label("Overview"),
                            Tab::new().label("Activity"),
                            Tab::new().label("Settings"),
                        ])
                        .on_click(cx.listener(|this, index, _, cx| {
                            this.tab = *index;
                            this.log(format!("Tab: {index}"), cx);
                        })),
                )
                .child(text(
                    [
                        "Your workspace at a glance.",
                        "Recent changes appear here.",
                        "Workspace preferences.",
                    ][self.tab],
                    TextStyle::Body,
                    cx,
                ))
                .child(
                    Breadcrumb::new()
                        .child(BreadcrumbItem::new("Workspace").on_click(
                            cx.listener(|this, _, _, cx| this.log("Breadcrumb: Workspace", cx)),
                        ))
                        .child(BreadcrumbItem::new("Projects"))
                        .child(BreadcrumbItem::new("Atlas")),
                )
                .child(
                    button("dropdown", "Project actions", ActionStyle::Secondary, cx)
                        .dropdown_caret(true)
                        .dropdown_menu(|menu, _, _| {
                            menu.item(PopupMenuItem::new("Edit project").on_click(
                                |_, window, cx| {
                                    window.push_notification("Edit project selected", cx)
                                },
                            ))
                            .item(
                                PopupMenuItem::new("Duplicate")
                                    .icon(IconName::Copy)
                                    .on_click(|_, window, cx| {
                                        window.push_notification("Project duplicated", cx)
                                    }),
                            )
                            .separator()
                            .item(
                                PopupMenuItem::new("Delete project").on_click(|_, window, cx| {
                                    window.open_dialog(cx, |d, _, _| {
                                        d.title("Delete project?")
                                            .child(
                                                "This demonstration does not delete any real data.",
                                            )
                                            .confirm()
                                    })
                                }),
                            )
                        }),
                )
                .child(
                    self.row(cx)
                        .child(
                            button("prev", "Previous", ActionStyle::Secondary, cx)
                                .disabled(self.page == 0)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.page = this.page.saturating_sub(1);
                                    this.log("Previous page", cx);
                                })),
                        )
                        .child(text(
                            format!("Page {} of 5", self.page + 1),
                            TextStyle::Body,
                            cx,
                        ))
                        .child(
                            button("next", "Next", ActionStyle::Secondary, cx)
                                .disabled(self.page == 4)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.page = (this.page + 1).min(4);
                                    this.log("Next page", cx);
                                })),
                        ),
                ),
            cx,
        )
    }
    fn overlays(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("06", "Overlays", "Open real overlays. Escape closes them and focus returns to the trigger.", self.row(cx)
            .child(button("modal", "Open modal", ActionStyle::Primary, cx).on_click(cx.listener(|this, _, window, cx| {
                let state = this.modal_name.clone(); window.open_dialog(cx, move |d, _, cx| {
                    let state = state.clone(); d.title("Create workspace").child(text("A place for your projects and people.", TextStyle::Caption, cx)).child(field("Name", input(&state, cx), cx)).confirm()
                        .on_ok(move |_, window, cx| { let value = state.read(cx).value().to_string();
                            if value.trim().is_empty() { window.push_notification((NotificationType::Error, "Enter a workspace name"), cx); return false; }
                            window.push_notification((NotificationType::Success, SharedString::from(format!("Created {value}"))), cx); true })
                });
            })))
            .child(button("confirm", "Confirmation", ActionStyle::Secondary, cx).on_click(|_, window, cx| window.open_dialog(cx, |d, _, _| d.title("Archive this project?").child("This is a component demonstration. No real project will be changed.").confirm().on_ok(|_, window, cx| { window.push_notification("Demo project archived", cx); true }))))
            .child(button("sheet", "Open drawer", ActionStyle::Secondary, cx).on_click(|_, window, cx| window.open_sheet(cx, |s, _, cx| s.title("Project details").child(v_flex().gap_4().child(text("Atlas", TextStyle::Heading, cx)).child(text("A side panel keeps context in view.", TextStyle::Body, cx)).child(button("close-sheet", "Done", ActionStyle::Primary, cx).on_click(|_, window, cx| window.close_sheet(cx)))))))
            .child(button("tooltip", "Hover for tooltip", ActionStyle::Quiet, cx).tooltip("Helpful context, right where you need it."))
            .child(button("toast", "Show notification", ActionStyle::Secondary, cx).on_click(|_, window, cx| window.push_notification((NotificationType::Success, "Changes saved"), cx))), cx)
    }
    fn feedback(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section(
            "07",
            "Feedback",
            "Status, validation and loading without unnecessary visual noise.",
            self.column(cx)
                .child(
                    self.row(cx)
                        .child(Tag::primary().child("In progress"))
                        .child(Tag::success().child("Complete"))
                        .child(Tag::warning().child("Pending"))
                        .child(Tag::secondary().child("Draft"))
                        .child(Tag::danger().child("Failed")),
                )
                .child(Alert::info("info", "Your changes are saved automatically."))
                .child(Alert::success("success", "Workspace created successfully."))
                .child(Alert::warning(
                    "warning",
                    "You are approaching your storage limit.",
                ))
                .child(Alert::error("error", "Unable to connect. Try again."))
                .child(self.row(cx).child(Spinner::new()).child(text(
                    "Loading projects…",
                    TextStyle::Caption,
                    cx,
                )))
                .child(
                    v_flex()
                        .gap_3()
                        .child(Skeleton::new().w(px(280.)).h(px(14.)))
                        .child(Skeleton::new().w(px(200.)).h(px(14.))),
                )
                .child(Progress::new().value(self.progress)),
            cx,
        )
    }
    fn data(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("08", "Data & structure", "A virtualized table, identity, disclosure and empty states.", self.column(cx)
            .child(div().h(px(300.)).min_h(px(300.)).flex_shrink_0().w_full().child(Table::new(&self.table).stripe(false).with_size(control_size(cx))))
            .child(self.row(cx).child(Avatar::new().name("Alex Chen")).child(Avatar::new().name("Daniel Park")).child(Avatar::new().name("Taylor Kim")))
            .child(Accordion::new("accordion").bordered(false).item(|i| i.title("How does theming work?").child("Change the theme configuration; every component consumes semantic tokens.")).item(|i| i.title("Can I override individual components?").child("Yes. Use the underlying native component builders and Rusty GPUI tokens.")))
            .child(empty_state("Nothing here yet", "Create a project to get started.", cx)).child(Divider::horizontal().label("End of collection")), cx)
    }
    fn lab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("09", "Performance lab", "Frame samples measure render-callback intervals, not GPU timings. Results depend on your machine.", self.column(cx)
            .when(self.category == 9, |d| d.child(div().h(px(300.)).min_h(px(300.)).flex_shrink_0().w_full().child(Table::new(&self.table).with_size(control_size(cx)))))
            .child(self.row(cx).children([100_usize, 1_000, 10_000].map(|rows| button(("rows", rows), format!("{rows} rows"), ActionStyle::Secondary, cx).on_click(cx.listener(move |this, _, _, cx| {
                this.table.update(cx, |state, cx| { state.delegate_mut().set_rows(rows); state.refresh(cx); }); this.log(format!("Table now has {rows} virtualized rows"), cx);
            })))))
            .child(button("benchmark", if self.benchmark_remaining > 0 { "Sampling frames…" } else { "Sample 120 frames" }, ActionStyle::Primary, cx).disabled(self.benchmark_remaining > 0).on_click(cx.listener(|this, _, _, cx| {
                this.frame_samples.clear(); this.benchmark_remaining = 121; this.benchmark_last = None; this.benchmark_result = "Sampling…".into(); cx.notify();
            })))
            .child(text(self.benchmark_result.clone(), TextStyle::Body, cx)).child(text("Scroll the table and change the theme while sampling. Use GPUI_MEASUREMENTS=1 for upstream instrumentation.", TextStyle::Caption, cx))
            .child(text("Interaction history", TextStyle::Subheading, cx)).children(self.events.iter().map(|event| text(event.clone(), TextStyle::Caption, cx)))
            .when(self.events.is_empty(), |d| d.child(text("Try a control to see its events here.", TextStyle::Caption, cx))), cx)
    }
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.benchmark_remaining > 0 {
            let now = Instant::now();
            if let Some(last) = self.benchmark_last {
                self.frame_samples
                    .push(now.duration_since(last).as_secs_f64() * 1000.);
            }
            self.benchmark_last = Some(now);
            self.benchmark_remaining -= 1;
            if self.benchmark_remaining == 0 {
                let avg =
                    self.frame_samples.iter().sum::<f64>() / self.frame_samples.len().max(1) as f64;
                let mut sorted = self.frame_samples.clone();
                sorted.sort_by(f64::total_cmp);
                let p95 = sorted
                    .get((sorted.len() as f64 * 0.95) as usize)
                    .copied()
                    .unwrap_or(0.);
                self.benchmark_result = format!(
                    "{} samples · mean {:.2} ms · p95 {:.2} ms",
                    sorted.len(),
                    avg,
                    p95
                );
            } else {
                window.request_animation_frame();
            }
        }
        let t = tokens(cx).clone();
        let sections: [(usize, &str); 9] = [
            (
                1,
                "foundations typography heading text color spacing tokens",
            ),
            (2, "actions buttons loading disabled tooltip"),
            (
                3,
                "inputs text field email password textarea search select combobox date picker",
            ),
            (
                4,
                "selection checkbox radio switch slider segmented progress",
            ),
            (5, "navigation tabs breadcrumb pagination dropdown menus"),
            (
                6,
                "overlays modal dialog confirmation drawer sheet toast notification tooltip",
            ),
            (
                7,
                "feedback alert badge tag skeleton spinner loading progress",
            ),
            (8, "data table avatar accordion divider empty state"),
            (9, "performance lab benchmark events virtualized rows"),
        ];
        let mut content = v_flex()
            .w_full()
            .gap_2()
            .when(window.viewport_size().width < px(850.), |this| {
                this.child(self.row(cx).children(CATEGORIES.iter().enumerate().map(
                    |(index, label)| {
                        button(("compact-category", index), *label, ActionStyle::Quiet, cx)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.category = index;
                                this.search
                                    .update(cx, |state, cx| state.set_value("", window, cx));
                                this.page_scroll.set_offset(point(px(0.), px(0.)));
                                cx.notify();
                            }))
                    },
                )))
            })
            .child(
                self.row(cx)
                    .child(
                        button("compact-window", "Compact window", ActionStyle::Quiet, cx)
                            .on_click(|_, window, _| window.resize(size(px(600.), px(650.)))),
                    )
                    .child(
                        button("wide-window", "Wide window", ActionStyle::Quiet, cx)
                            .on_click(|_, window, _| window.resize(size(px(1440.), px(900.)))),
                    )
                    .when(window.viewport_size().width < px(1100.), |this| {
                        this.child(
                            button(
                                "theme-inspector",
                                if self.compact_theme_visible {
                                    "Hide theme inspector"
                                } else {
                                    "Theme inspector"
                                },
                                ActionStyle::Secondary,
                                cx,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.compact_theme_visible = !this.compact_theme_visible;
                                cx.notify();
                            })),
                        )
                    }),
            )
            .when(
                self.compact_theme_visible && window.viewport_size().width < px(1100.),
                |this| this.child(self.theme_controls(cx)),
            )
            .child(text(
                "RUSTY GPUI / COMPONENT LIBRARY",
                TextStyle::Caption,
                cx,
            ))
            .child(text("Primitives, precisely.", TextStyle::Display, cx))
            .child(text(
                "A considered foundation for whatever comes next.",
                TextStyle::Body,
                cx,
            ))
            .child(
                div().mt_4().w_full().max_w(px(500.)).child(
                    input(&self.search, cx)
                        .prefix(Icon::new(IconName::Search))
                        .cleanable(true),
                ),
            );
        let mut count = 0;
        for (category, keywords) in sections {
            if !self.visible(category, keywords, cx) {
                continue;
            }
            count += 1;
            let section = match category {
                1 => self.foundations(cx).into_any_element(),
                2 => self.actions(cx).into_any_element(),
                3 => self.inputs(cx).into_any_element(),
                4 => self.selection(cx).into_any_element(),
                5 => self.navigation(cx).into_any_element(),
                6 => self.overlays(cx).into_any_element(),
                7 => self.feedback(cx).into_any_element(),
                8 => self.data(cx).into_any_element(),
                _ => self.lab(cx).into_any_element(),
            };
            content = content.child(section);
        }
        if count == 0 {
            content = content.child(empty_state(
                "No matching components",
                "Clear the search or choose another category.",
                cx,
            ));
        }
        // Opt-in native smoke-test instrumentation. Never enabled in normal use.
        #[cfg(debug_assertions)]
        if let Some(path) = std::env::var_os("RUSTY_GPUI_SMOKE_STATE") {
            let state = serde_json::json!({"category":self.category,"clicks":self.clicks,"checked":self.checked,"switched":self.switched,
                "radio":self.radio,"tab":self.tab,"page":self.page,"loading":self.loading,"progress":self.progress,
                "rows":self.table.read(cx).delegate().rows,"benchmark":self.benchmark_result,"remaining":self.benchmark_remaining,
                "page_offset": f32::from(self.page_scroll.offset().y),
                "table_offset": f32::from(self.table.read(cx).vertical_scroll_handle.0.borrow().base_handle.offset().y),
                "table_x": f32::from(self.table.read(cx).horizontal_scroll_handle.offset().x),
                "project_results": project_matches(&self.demo_search.read(cx).value()), "collections":count,
                "disabled_focused":self.disabled_input.focus_handle(cx).is_focused(window),
                "disabled_value":self.disabled_input.read(cx).value().to_string(),
                "focus_field": if self.name.focus_handle(cx).is_focused(window) {"name"} else if self.email.focus_handle(cx).is_focused(window) {"email"} else {"other"},
                "theme":settings::value(&self.config),"dialog":window.has_active_dialog(cx),"sheet":window.has_active_sheet(cx),"events":self.events});
            let _ = std::fs::write(path, state.to_string());
        }
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);
        div()
            .relative()
            .size_full()
            .child(
                h_flex()
                    .size_full()
                    .bg(color(t.background))
                    .text_color(color(t.foreground))
                    .font_family(t.font_family.clone())
                    .text_size(px(t.font_size))
                    .items_start()
                    .when(window.viewport_size().width >= px(850.), |this| {
                        this.child(self.sidebar(cx))
                    })
                    .child(
                        div()
                            .id("gallery-scroll")
                            .flex_1()
                            .min_w_0()
                            .h_full()
                            .overflow_y_scroll()
                            .track_scroll(&self.page_scroll)
                            .p(px(t.spacing[6]))
                            .child(content),
                    )
                    .when(window.viewport_size().width >= px(1100.), |this| {
                        this.child(
                            div()
                                .id("settings-scroll")
                                .h_full()
                                .overflow_y_scroll()
                                .child(self.theme_controls(cx)),
                        )
                    }),
            )
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
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
            apply_theme(settings::load(), None, cx);
            let display = cx
                .primary_display()
                .map(|d| d.bounds().size)
                .unwrap_or(size(px(1440.), px(960.)));
            let bounds = Bounds::centered(
                None,
                size(
                    px(1440.).min(display.width),
                    px(960.).min(display.height - px(40.)),
                ),
                cx,
            );
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Rusty GPUI — Component Library".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let gallery = cx.new(|cx| Gallery::new(window, cx));
                    cx.new(|cx| Root::new(gallery, window, cx))
                },
            )
            .expect("Unable to open the GPUI gallery window");
            cx.activate(true);
        });
}

#[cfg(test)]
mod table_tests {
    use super::ProjectTable;
    #[test]
    fn moving_columns_keeps_headers_and_values_associated() {
        let mut table = ProjectTable::new();
        let before: Vec<_> = (0..3).map(|col| table.cell_value(0, col)).collect();
        table.reorder_column(0, 2);
        assert_eq!(
            table
                .columns
                .iter()
                .map(|col| col.key.as_ref())
                .collect::<Vec<&str>>(),
            vec!["status", "owner", "name"]
        );
        assert_eq!(table.cell_value(0, 0), before[1]);
        assert_eq!(table.cell_value(0, 1), before[2]);
        assert_eq!(table.cell_value(0, 2), before[0]);
        table.descending = true;
        table.set_rows(10000);
        assert_eq!(table.cell_value(0, 2), "Evergreen / 10000");
    }
    #[test]
    fn project_search_filters_case_insensitively_and_handles_empty_results() {
        assert_eq!(super::project_matches("  beAC  "), vec!["Beacon"]);
        assert_eq!(super::project_matches("").len(), 5);
        assert!(super::project_matches("missing project").is_empty());
    }
    #[test]
    fn collection_search_crosses_the_selected_category() {
        assert!(super::collection_matches(
            6,
            2,
            "overlays modal drawer",
            " DRAWER "
        ));
        assert!(!super::collection_matches(
            6,
            2,
            "overlays modal drawer",
            ""
        ));
        assert!(super::collection_matches(
            6,
            2,
            "overlays modal drawer",
            "modal drawer"
        ));
        assert!(!super::collection_matches(
            6,
            2,
            "overlays modal drawer",
            "modal unknown"
        ));
    }
    #[test]
    fn virtualized_data_sorts_all_rows_by_name_and_preserves_row_ids() {
        let mut table = ProjectTable::new();
        assert!(
            table
                .order
                .windows(2)
                .all(|w| (w[0] % 5, w[0]) <= (w[1] % 5, w[1]))
        );
        table.descending = true;
        table.set_rows(10_000);
        assert_eq!(table.order.len(), 10_000);
        assert!(
            table
                .order
                .windows(2)
                .all(|w| (w[0] % 5, w[0]) >= (w[1] % 5, w[1]))
        );
        let unique: std::collections::HashSet<_> = table.order.iter().collect();
        assert_eq!(unique.len(), 10_000);
        assert!(table.order.iter().all(|id| *id < 10_000));
    }
}
