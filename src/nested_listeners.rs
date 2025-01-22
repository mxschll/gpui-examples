use gpui::{
    div, px, rgba, size, App, AppContext, Bounds, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, Styled, ViewContext, VisualContext, WindowBounds, WindowOptions,
};

struct ParentView {}

impl Render for ParentView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("parent")
            .bg(rgba(0x00ff00ff))
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .text_xl()
            .child(div().pb_8().pt_8().flex().items_center().child("Parent"))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|_this, _event, _cx| {
                    println!("Parent Clicked");
                }),
            )
            .child(cx.new_view(|_cx| ChildView {}))
    }
}

struct ChildView {}

impl Render for ChildView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("child")
            .bg(rgba(0xff0000ff))
            .flex()
            .justify_center()
            .items_center()
            .h(px(100.0))
            .w(px(100.0))
            .text_xl()
            .hover(|x| x.bg(rgba(0xffffffff)))
            .child("Child")
            // The following does NOT work using cx.listener
            // .on_mouse_up(
            //     MouseButton::Left,
            //     cx.listener(|_this, _event, cx| {
            //         cx.stop_propagation();
            //         println!("Child Clicked");
            //     }),
            // ),
            .on_mouse_up(MouseButton::Left, move |_event, cx| {
                cx.stop_propagation();
                println!("Child Clicked");
            })
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
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
                |cx| cx.new_view(|_cx| ParentView {}),
            )
            .unwrap();

        window
            .update(cx, |_view, cx| {
                cx.activate(true);
            })
            .unwrap();
    });
}
