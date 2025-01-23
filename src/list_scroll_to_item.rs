use gpui::{
    div, prelude::*, px, rgb, size, uniform_list, App, AppContext, Bounds, ScrollStrategy, Timer,
    UniformListScrollHandle, View, ViewContext, WindowBounds, WindowOptions,
};
use std::time::Duration;

const LIST_ITEMS: usize = 50;
const WINDOW_WIDTH: f32 = 300.0;
const WINDOW_HEIGHT: f32 = 200.0;
const UPDATE_INTERVAL: Duration = Duration::from_millis(500);

struct ListView {
    list_scroll_handle: UniformListScrollHandle,
    selected_index: usize,
}

impl ListView {
    fn new(cx: &mut ViewContext<ContainerView>) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            Self::spanw_auto_scroll_task(cx);
            Self {
                list_scroll_handle: UniformListScrollHandle::new(),
                selected_index: 0,
            }
        })
    }

    /// Spawns an async task that automatically scrolls the list
    fn spanw_auto_scroll_task(cx: &mut ViewContext<Self>) {
        cx.spawn(|this, mut cx| async move {
            loop {
                Timer::after(UPDATE_INTERVAL).await;
                if let Some(view) = this.upgrade() {
                    cx.update_view(&view, |view, cx| {
                        view.selected_index = (view.selected_index + 1) % LIST_ITEMS;
                        view.list_scroll_handle
                            .scroll_to_item(view.selected_index, ScrollStrategy::Top);

                        cx.notify();
                    })
                    .ok();
                }
            }
        })
        .detach();
    }
}

struct ContainerView {
    scroll_list: View<ListView>,
}

impl Render for ContainerView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xffffff))
            .size_full()
            .text_xl()
            .text_color(rgb(0x000000))
            .child(self.scroll_list.clone())
    }
}

impl Render for ListView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0xffffff)).child(
            uniform_list(
                cx.view().clone(),
                "entries",
                LIST_ITEMS,
                |this, range, _cx| {
                    let mut items = Vec::new();
                    for ix in range {
                        let item = ix + 1;
                        let is_selected = this.selected_index == ix;

                        items.push(
                            div()
                                .id(ix)
                                .px_2()
                                .child(format!("Item {item}"))
                                .when(is_selected, |x| x.bg(rgb(0xff0000))),
                        );
                    }
                    items
                },
            )
            .track_scroll(self.list_scroll_handle.clone())
            .h_full(),
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
                cx.new_view(|cx| {
                    let scroll_list = ListView::new(cx);
                    ContainerView { scroll_list }
                })
            },
        )
        .unwrap();
    });
}
