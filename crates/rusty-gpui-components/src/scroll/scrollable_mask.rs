use gpui::{
    px, relative, App, Axis, BorderStyle, Bounds, ContentMask, Corners, Edges, Element, ElementId,
    GlobalElementId, Hitbox, Hsla, IntoElement, IsZero as _, LayoutId, PaintQuad, Pixels, Point,
    Position, ScrollHandle, ScrollWheelEvent, Style, Window,
};

use crate::AxisExt;

/// Make a scrollable mask element to cover the parent view with the mouse wheel event listening.
///
/// When the mouse wheel is scrolled, will move the `scroll_handle` scrolling with the `axis` direction.
/// You can use this `scroll_handle` to control what you want to scroll.
/// This is only can handle once axis scrolling.
pub struct ScrollableMask {
    axis: Axis,
    scroll_handle: ScrollHandle,
    cross_axis_scroll_handle: Option<ScrollHandle>,
    contain_scroll: bool,
    debug: Option<Hsla>,
}

impl ScrollableMask {
    /// Create a new scrollable mask element.
    pub fn new(axis: Axis, scroll_handle: &ScrollHandle) -> Self {
        Self {
            scroll_handle: scroll_handle.clone(),
            cross_axis_scroll_handle: None,
            contain_scroll: false,
            axis,
            debug: None,
        }
    }

    /// Route the perpendicular wheel direction to a nested scroll viewport.
    pub fn cross_axis_scroll_handle(mut self, handle: &ScrollHandle) -> Self {
        self.cross_axis_scroll_handle = Some(handle.clone());
        self
    }

    /// Keep wheel gestures inside this viewport, including at its boundaries.
    pub fn contain_scroll(mut self, contain: bool) -> Self {
        self.contain_scroll = contain;
        self
    }

    /// Enable the debug border, to show the mask bounds.
    #[allow(dead_code)]
    pub fn debug(mut self) -> Self {
        self.debug = Some(gpui::yellow());
        self
    }
}

impl IntoElement for ScrollableMask {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ScrollableMask {
    type RequestLayoutState = ();
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        // Set the layout style relative to the table view to get same size.
        style.position = Position::Absolute;
        style.flex_grow = 1.0;
        style.flex_shrink = 1.0;
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();

        (window.request_layout(style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        _: &mut App,
    ) -> Self::PrepaintState {
        // Move y to bounds height to cover the parent view.
        let cover_bounds = Bounds {
            origin: Point {
                x: bounds.origin.x,
                y: bounds.origin.y - bounds.size.height,
            },
            size: bounds.size,
        };

        window.insert_hitbox(cover_bounds, gpui::HitboxBehavior::Normal)
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        _: &mut App,
    ) {
        let is_horizontal = self.axis.is_horizontal();
        let line_height = window.line_height();
        let bounds = hitbox.bounds;

        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            if let Some(color) = self.debug {
                window.paint_quad(PaintQuad {
                    bounds,
                    border_widths: Edges::all(px(1.0)),
                    border_color: color,
                    background: gpui::transparent_white().into(),
                    corner_radii: Corners::all(px(0.)),
                    border_style: BorderStyle::default(),
                });
            }

            window.on_mouse_event({
                let view_id = window.current_view();
                let scroll_handle = self.scroll_handle.clone();
                let cross_axis_scroll_handle = self.cross_axis_scroll_handle.clone();
                let contain_scroll = self.contain_scroll;

                move |event: &ScrollWheelEvent, phase, _, cx| {
                    if !(bounds.contains(&event.position) && phase.bubble()) {
                        return;
                    }

                    let mut delta = event.delta.pixel_delta(line_height);

                    // Limit for only one way scrolling at same time.
                    // When use MacBook touchpad we may get both x and y delta,
                    // only allows the one that more to scroll.
                    if !delta.x.is_zero() && !delta.y.is_zero() {
                        if delta.x.abs() > delta.y.abs() {
                            delta.y = px(0.);
                        } else {
                            delta.x = px(0.);
                        }
                    }

                    let cross_axis = if is_horizontal { !delta.y.is_zero() } else { !delta.x.is_zero() };
                    let (handle, horizontal) = if cross_axis {
                        match cross_axis_scroll_handle.as_ref() {
                            Some(handle) => (handle, !is_horizontal),
                            None => return,
                        }
                    } else { (&scroll_handle, is_horizontal) };
                    let mut offset = handle.offset();
                    let maximum = handle.max_offset();
                    if horizontal {
                        offset.x = (offset.x + delta.x).clamp(-maximum.width.max(px(0.)), px(0.));
                    } else {
                        offset.y = (offset.y + delta.y).clamp(-maximum.height.max(px(0.)), px(0.));
                    }
                    let before = handle.offset();
                    handle.set_offset(offset);
                    if handle.offset() != before {
                        cx.notify(view_id);
                    }
                    if contain_scroll || handle.offset() != before {
                        cx.stop_propagation();
                    }
                }
            });
        });
    }
}
