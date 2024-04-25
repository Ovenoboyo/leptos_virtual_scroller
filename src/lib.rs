use leptos::{
    component, create_effect, create_node_ref, create_rw_signal, event_target, leptos_dom::logging::console_log, view, CollectView, IntoView, SignalGet, SignalGetUntracked, SignalSet
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

    let inner_height = create_rw_signal(0usize);

    let window_height = create_rw_signal(0);

    let scroll_top = create_rw_signal(0);
    let start_index = create_rw_signal(0);
    let end_index = create_rw_signal(0);

    create_effect(move |_| {
        let items_len = each.get().len();
        items_len_sig.set(items_len);
        inner_height.set(items_len * item_height);
    });

    create_effect(move |_| {
        let scroll_top = scroll_top.get();
        let window_height = window_height.get();
        let items_len = items_len_sig.get();

        let start_index_res = scroll_top / item_height;
        let end_index_res = ((scroll_top + window_height) / item_height).min(items_len);

        if start_index_res != start_index.get_untracked() {
            start_index.set(start_index_res);
        }

        if end_index_res != end_index.get_untracked() {
            end_index.set(end_index_res);
        }
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

                {move || {
                    let items_len = items_len_sig.get();
                    let start_index = start_index.get();
                    let end_index = (end_index.get() + 2).min(items_len);
                    let buffer_start = if start_index >= 2 { start_index - 2 } else { start_index };
                    let buffer_end = (end_index + 2).min(items_len);
                    let mut ret = vec![];
                    let each = each.get();
                    for i in buffer_start..buffer_end {
                        let item = each
                            .get(i)
                            .unwrap_or_else(|| {
                                panic!("Item passed to VirtualScroller at index {} should exist", i)
                            });
                        ret.push(
                            view! {
                                <div
                                    style=format!(
                                        "position: absolute; width: 100%; {}",
                                        inner_el_style,
                                    )

                                    style:top=format!("{}px", i * item_height)
                                >

                                    {children((i, item))}

                                </div>
                            },
                        );
                    }
                    ret.collect_view()
                }}

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
    let items_len_sig = create_rw_signal(0usize);

    let inner_height = create_rw_signal(0usize);

    let window_height = create_rw_signal(0);
    let window_width = create_rw_signal(0);


    let scroll_top = create_rw_signal(0);
    let start_index = create_rw_signal(0);
    let end_index = create_rw_signal(0);
    let grid_items = create_rw_signal(0);

    create_effect(move |_| {
        let items_len = each.get().len();

        
        items_len_sig.set(items_len);
    });

    create_effect(move |_| {
        let grid_items = grid_items.get();
        if grid_items == 0 {
            return;
        }
        let items_len = items_len_sig.get();
        inner_height.set((items_len / grid_items) * item_height);
    });

    create_effect(move |_| {
        let scroll_top = scroll_top.get();
        let window_height = window_height.get();
        let window_width = window_width.get();
        let items_len = items_len_sig.get();

        let grid_items_res = window_width / item_width;
        grid_items.set(grid_items_res);
        console_log(format!("grid items {}", grid_items_res).as_str());

        let start_index_res = (scroll_top / item_height) * grid_items_res;
        let end_index_res = (((scroll_top + window_height) / item_height) * grid_items_res).min(items_len);

        if start_index_res != start_index.get_untracked() {
            start_index.set(start_index_res);
        }

        if end_index_res != end_index.get_untracked() {
            end_index.set(end_index_res);
        }
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

                {move || {
                    let grid_items = grid_items.get().max(1);
                    let extra_items = grid_items * 1;
                    let items_len = items_len_sig.get();
                    let start_index = start_index.get();
                    let end_index = (end_index.get() + extra_items).min(items_len);
                    let buffer_start = if start_index >= extra_items { start_index - extra_items } else { start_index };
                    let buffer_end = (end_index + extra_items).min(items_len);
        
                    let mut ret = vec![];
                    let each = each.get();
                    for i in buffer_start..buffer_end {
                        let grid_index = i % grid_items;
                        let item = each
                            .get(i)
                            .unwrap_or_else(|| {
                                panic!("Item passed to VirtualScroller at index {} should exist", i)
                            });
                        ret.push(
                            view! {
                                <div
                                    style=format!(
                                        "position: absolute; {}",
                                        inner_el_style,
                                    )

                                    style:top=format!("{}px", ((i) / grid_items) * item_height)
                                    style:left=format!("{}px", grid_index * item_width)
                                >

                                    {children((i, item))}

                                </div>
                            },
                        );
                    }
                    ret.collect_view()
                }}

            </div>
        </div>
    }
}