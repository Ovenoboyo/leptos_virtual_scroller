use std::rc::Rc;

use leptos::{
    component, create_effect, create_memo, create_node_ref, create_rw_signal, event_target,
    leptos_dom::logging::console_log, view, CollectView, For, IntoView, SignalGet,
    SignalGetUntracked, SignalSet, SignalUpdate,
};
use leptos_use::use_resize_observer;

#[component]
pub fn VirtualScroller<T, S, C, N>(
    #[prop()] each: S,
    #[prop()] children: C,
    #[prop()] item_height: usize,
    #[prop(default = "")] inner_el_style: &'static str,
) -> impl IntoView
where
    C: Fn((usize, &T)) -> N + 'static,
    N: IntoView,
    S: SignalGet<Value = Vec<T>> + Copy + 'static,
{
    let items_len_sig = create_rw_signal(0usize);

    let inner_height = create_memo(move |_| {
        let items_len = each.get().len();
        items_len_sig.set(items_len);
        items_len * item_height
    });

    let window_height = create_rw_signal(0);

    let scroll_top = create_rw_signal(0);

    let index_bounds = create_memo(move |_| {
        let scroll_top = scroll_top.get();
        let window_height = window_height.get();
        let items_len = items_len_sig.get();

        let start_index_res = scroll_top / item_height;
        let end_index_res = ((scroll_top + window_height) / item_height).min(items_len);

        (start_index_res, end_index_res)
    });

    let buffer_bounds = create_memo(move |_| {
        let items_len = items_len_sig.get();
        let (start_index, end_index) = index_bounds.get();
        let buffer_start = if start_index >= 2 {
            start_index - 2
        } else {
            start_index
        };
        let buffer_end = (end_index + 2).min(items_len);
        (buffer_start, buffer_end)
    });

    let container = create_node_ref();
    use_resize_observer(container, move |a, b| {
        let rect = a[0].content_rect();
        window_height.set(rect.height() as usize)
    });

    view! {
        <div
            ref=container
            style="width: 100%; height: 100%; overflow-y: scroll;"
            on:scroll=move |ev| {
                let target: leptos::web_sys::HtmlElement = event_target(&ev);
                scroll_top.set(target.scroll_top() as usize);
            }
        >

            <div
                id="scroller"
                style="position: relative;"
                style:height=move || format!("{}px", inner_height.get())
            >

                <For
                    each=move || (buffer_bounds.get().0..buffer_bounds.get().1)
                    key=|i| *i
                    children=move |i| {
                        view! {
                            <div
                                style=format!("position: absolute; width: 100%; {}", inner_el_style)

                                style:top=format!("{}px", i * item_height)
                            >

                                {children((
                                    i,
                                    each
                                        .get()
                                        .get(i)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "Item passed to VirtualScroller at index {} should exist",
                                                i,
                                            )
                                        }),
                                ))}

                            </div>
                        }
                    }
                />

            </div>
        </div>
    }
}

#[component]
pub fn VirtualGridScroller<T, S, C, N>(
    #[prop()] each: S,
    #[prop()] children: C,
    #[prop()] item_height: usize,
    #[prop()] item_width: usize,
    #[prop(default = "")] inner_el_style: &'static str,
) -> impl IntoView
where
    C: Fn((usize, &T)) -> N + 'static,
    N: IntoView,
    S: SignalGet<Value = Vec<T>> + Copy + 'static,
{
    let items_len_sig = create_memo(move |_| each.get().len());
    let window_height = create_rw_signal(0);
    let window_width = create_rw_signal(0);

    let grid_items = create_memo(move |_| {
        let window_width = window_width.get();
        (window_width / item_width).max(1)
    });

    let inner_height = create_memo(move |_| {
        let grid_items = grid_items.get();
        if grid_items == 0 {
            return 0;
        }
        let items_len = items_len_sig.get();
        (items_len / grid_items) * item_height
    });

    let scroll_top = create_rw_signal(0);

    let index_bounds = create_memo(move |_| {
        let scroll_top = scroll_top.get();
        let window_height = window_height.get();
        let items_len = items_len_sig.get();

        let grid_items_res = grid_items.get();

        let start_index_res = (scroll_top / item_height) * grid_items_res;
        let end_index_res =
            (((scroll_top + window_height) / item_height) * grid_items_res).min(items_len);

        (start_index_res, end_index_res)
    });

    let buffer_bounds = create_memo(move |_| {
        let grid_items = grid_items.get().max(1);
        let extra_items = grid_items * 1;
        let items_len = items_len_sig.get();
        let (start_index, end_index) = index_bounds.get();
        let end_index = (end_index + extra_items).min(items_len);
        let buffer_start = if start_index >= extra_items {
            start_index - extra_items
        } else {
            start_index
        };
        let buffer_end = (end_index + extra_items).min(items_len);
        (buffer_start, buffer_end)
    });

    let container = create_node_ref();
    use_resize_observer(container, move |a, b| {
        let rect = a[0].content_rect();
        window_height.set(rect.height() as usize);
        window_width.set(rect.width() as usize);
    });

    view! {
        <div
            ref=container
            style="width: 100%; height: 100%; overflow-y: scroll;"
            on:scroll=move |ev| {
                let target: leptos::web_sys::HtmlElement = event_target(&ev);
                scroll_top.set(target.scroll_top() as usize);
            }
        >

            <div
                id="scroller"
                style="position: relative;"
                style:height=move || format!("{}px", inner_height.get())
            >

                <For
                    each=move || (buffer_bounds.get().0..buffer_bounds.get().1)
                    key=|i| *i
                    children=move |i| {
                        let grid_index = i % grid_items.get();
                        view! {
                            <div
                                style=format!("position: absolute; {}", inner_el_style)

                                style:top=format!("{}px", ((i) / grid_items.get()) * item_height)
                                style:left=format!("{}px", grid_index * item_width)
                            >

                                {children((
                                    i,
                                    each
                                        .get()
                                        .get(i)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "Item passed to VirtualScroller at index {} should exist",
                                                i,
                                            )
                                        }),
                                ))}

                            </div>
                        }
                    }
                />

            </div>
        </div>
    }
}
