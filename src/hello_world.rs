use gpui::{
    div, prelude::*, px, rgb, size, App, AppContext, Bounds, ViewContext, WindowBounds,
    WindowOptions,
};

struct View;

impl Render for View {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0x000000))
            .child("Hello World")
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| View {}),
        )
        .unwrap();
    });
}
