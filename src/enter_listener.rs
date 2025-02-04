//! This example demonstrates a simple interactive GPUI application that:
//! - Creates a window with a counter
//! - Handles keyboard input (Enter key)
//! - Updates state and re-renders on user interaction

use gpui::{
    actions, div, prelude::*, px, rgb, size, App, Application, Bounds, Context, FocusHandle,
    Focusable, KeyBinding, Window, WindowBounds, WindowOptions,
};

actions!(text_input, [Enter]);

struct SimpleView {
    counter: i32,
    focus_handle: FocusHandle,
}

impl SimpleView {
    fn handle_enter(&mut self, _: &Enter, _window: &mut Window, cx: &mut Context<Self>) {
        self.counter += 1;
        cx.notify();
    }
}

// FocusableView is needed for .on_action to work
impl Focusable for SimpleView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SimpleView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xaaaaaa))
            .track_focus(&self.focus_handle(cx)) // Required for .on_action to work
            .on_action(cx.listener(Self::handle_enter))
            .size_full()
            .text_xl()
            .flex()
            .justify_center()
            .items_center()
            .child(format!("Press Enter! Count: {}", self.counter))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Bind the enter key
        cx.bind_keys([KeyBinding::new("enter", Enter, None)]);

        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(300.0), px(300.0)),
                        cx,
                    ))),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| SimpleView {
                        counter: 0,
                        focus_handle: cx.focus_handle(),
                    })
                },
            )
            .unwrap();

        // Focus the window so it can receive key events
        window
            .update(cx, |_view, window, cx| {
                cx.activate(true);
                cx.focus_self(window)
            })
            .unwrap();
    });
}
