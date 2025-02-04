use gpui::{
    div, px, rgba, size, App, AppContext, Application, Bounds, Context, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions,
};

struct ParentView {}

impl Render for ParentView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                cx.listener(|_this, _event, _, _cx| {
                    println!("Parent Clicked");
                }),
            )
            .child(cx.new(|_cx| ChildView {}))
    }
}

struct ChildView {}

impl Render for ChildView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
            .on_mouse_up(MouseButton::Left, move |_event, _, cx| {
                cx.stop_propagation();
                println!("Child Clicked");
            })
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
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
                |_, cx| cx.new(|_cx| ParentView {}),
            )
            .unwrap();

        window
            .update(cx, |_view, _, cx| {
                cx.activate(true);
            })
            .unwrap();
    });
}
