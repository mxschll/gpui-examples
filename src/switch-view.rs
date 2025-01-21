//! This example demonstrates a simple view-switching UI using GPUI.
//! It shows how to create a container with two different colored boxes (views)
//! and a button that switches between them.

use gpui::{
    div, prelude::*, px, rgb, size, AnyView, App, AppContext, Bounds, MouseButton, ViewContext,
    WindowBounds, WindowOptions,
};

const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 500.0;
const BOX_HEIGHT: f32 = 450.0;
const BUTTON_HEIGHT: f32 = 50.0;

mod boxes {
    use super::*;

    pub struct ColorBox1;
    pub struct ColorBox2;

    impl Render for ColorBox1 {
        fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
            color_box(rgb(0x50A050), "ColorBox1")
        }
    }

    impl Render for ColorBox2 {
        fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
            color_box(rgb(0xA05050), "ColorBox2")
        }
    }

    fn color_box(color: gpui::Rgba, label: &'static str) -> impl IntoElement {
        div()
            .w(px(WINDOW_WIDTH))
            .h(px(BOX_HEIGHT))
            .bg(color)
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .child(label)
    }
}

#[derive(Default)]
enum ActiveView {
    #[default]
    First,
    Second,
}

struct Container {
    active_view: ActiveView,
}

impl Container {
    fn switch_view(&mut self, _: &mut ViewContext<Self>) {
        self.active_view = match self.active_view {
            ActiveView::First => ActiveView::Second,
            ActiveView::Second => ActiveView::First,
        };
    }
}

impl Render for Container {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(match self.active_view {
                ActiveView::First => AnyView::from(cx.new_view(|_cx| boxes::ColorBox1)),
                ActiveView::Second => AnyView::from(cx.new_view(|_cx| boxes::ColorBox2)),
            })
            .child(
                div()
                    .bg(rgb(0x303030))
                    .h(px(BUTTON_HEIGHT))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(0x404040)))
                    .flex()
                    .justify_center()
                    .items_center()
                    .text_color(rgb(0xffffff))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _event, cx| {
                            this.switch_view(cx);
                            cx.notify(); // Indicate that this view has changed
                        }),
                    )
                    .child("Switch View"),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| Container {
                    active_view: ActiveView::default(),
                })
            },
        )
        .unwrap();
    });
}
