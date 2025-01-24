//! A simple counter application demonstrating GPUI's event handling.
//!
//! Event flow:
//! 1. Button click emits `Change` event in `CounterView::render()`
//! 2. Main window subscribes to these events during setup
//! 3. Subscriber updates the count and triggers re-render

use gpui::{
    div, prelude::*, rgb, App, AppContext, EventEmitter, MouseButton, ViewContext, WindowOptions,
};

struct CounterView {
    count: usize,
}

impl Render for CounterView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xF8FAFC))
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .text_3xl()
                    .text_color(rgb(0x1E293B))
                    .child(format!("Count: {}", self.count)),
            )
            .child(
                div()
                    .py_2()
                    .px_6()
                    .flex()
                    .text_2xl()
                    .items_center()
                    .rounded_full()
                    .bg(rgb(0x3B82F6))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(0x60A5FA)))
                    .child("Increment +2")
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|_this, _event, cx| {
                            cx.emit(Change { increment: 2 });
                        }),
                    ),
            )
    }
}

impl EventEmitter<Change> for CounterView {}

#[derive(Debug)]
struct Change {
    increment: usize,
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                ..Default::default()
            },
            |cx| {
                let view = cx.new_view(|_cx| CounterView { count: 0 });

                // Subscribes to events emitted by CounterView
                cx.subscribe(&view, |view, event, cx| {
                    view.update(cx, |view, cx| {
                        view.count += event.increment;
                        cx.notify();
                    });
                })
                .detach();

                view
            },
        )
        .unwrap();
    });
}
