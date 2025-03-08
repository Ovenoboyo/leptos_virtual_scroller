use std::hash::Hash;

use leptos::{
    component,
    html::Div,
    prelude::{
        event_target, Effect, ElementChild, For, Get, GlobalAttributes, IntoAny, Memo, NodeRef,
        NodeRefAttribute, OnAttribute, RwSignal, Set, StyleAttribute, Update, With,
    },
    view, IntoView,
};
use leptos_dom::logging::console_log;
use leptos_use::use_resize_observer;

struct ItemKey<S> {
    key: String,
    item: S,
}

impl<S> PartialEq for ItemKey<S> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

#[component]
pub fn VirtualScroller<T, S, K, KN, C, N, H>(
    #[prop()] each: S,
    #[prop()] key: KN,
    #[prop()] children: C,
    #[prop(optional)] header: Option<H>,
    #[prop(optional)] header_height: usize,
    #[prop()] item_height: usize,
    #[prop(default = "")] inner_el_style: &'static str,
    #[prop(optional)] node_ref: Option<NodeRef<Div>>,
) -> impl IntoView
where
    C: Fn((usize, &T)) -> N + 'static + Clone + Send + Sync,
    KN: (Fn((usize, &T)) -> K) + 'static + Clone + Send + Sync,
    K: Eq + Hash + 'static,
    N: IntoView + 'static,
    S: With<Value = Vec<T>> + Copy + 'static + Send + Sync,
    H: IntoView,
{
    let items_len_sig = RwSignal::new(0usize);
    let inner_height = Memo::new(move |_| {
        let items_len = each.with(|i| i.len());
        items_len_sig.set(items_len);
        items_len * item_height
    });

    let window_height = RwSignal::new(0);

    let scroll_top = RwSignal::new(0);

    let index_bounds = Memo::new(move |_| {
        let scroll_top = scroll_top.get();
        let window_height = window_height.get();
        let items_len = items_len_sig.get();

        let start_index_res = if scroll_top <= header_height {
            0
        } else {
            (scroll_top - header_height) / item_height
        };
        let end_index_res =
            ((header_height + scroll_top + window_height) / item_height).min(items_len);

        (start_index_res, end_index_res)
    });

    let buffer_bounds = Memo::new(move |_| {
        let items_len = items_len_sig.get();
        let (start_index, end_index) = index_bounds.get();
        let buffer_start = if start_index >= 2 { start_index - 2 } else { 0 };
        let buffer_end = (end_index + 2).min(items_len);
        (buffer_start, buffer_end)
    });

    let container = if let Some(node_ref) = node_ref {
        node_ref
    } else {
        NodeRef::new()
    };

    use_resize_observer(container, move |a, b| {
        let rect = a[0].content_rect();
        window_height.set(rect.height() as usize)
    });

    let force_refresh = RwSignal::new(false);
    Effect::new(move || {
        each.with(|_| {});
        force_refresh.update(|v| {
            *v = !*v;
        });
    });

    view! {
            <div
                node_ref=container
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

                {header}
    {
        move || {
            let children = children.clone();
            let key = key.clone();
            force_refresh.get();
            view!{
                <For each=move || (buffer_bounds.get().0..buffer_bounds.get().1) key=move |i| {
                    each.with(|item| {
                        key((*i, item.get(*i).unwrap()))
                    })
                } children=move |i| {
                    each.with(|item| {
                        let item = item.get(i).unwrap();
                        let (buffer_start, buffer_end) = buffer_bounds.get();
                        if i >= buffer_start && i <= buffer_end {
                            view! {
                                <div
                                    style=format!(
                                        "position: absolute; width: 100%; {}",
                                        inner_el_style,
                                    )

                                    style:top=format!("{}px", i * item_height + header_height)
                                >

                                    {children((i, item))}

                                </div>
                            }.into_any()
                        } else {
                            ().into_any()
                        }
                    })
                } />
            }
        }
    }
                </div>
            </div>
        }
}

#[component]
pub fn VirtualGridScroller<T, S, K, KN, C, N>(
    #[prop()] each: S,
    #[prop()] key: KN,
    #[prop()] children: C,
    #[prop()] item_height: usize,
    #[prop()] item_width: usize,
    #[prop(default = "")] inner_el_style: &'static str,
    #[prop(optional)] node_ref: Option<NodeRef<Div>>,
) -> impl IntoView
where
    C: Fn((usize, &T)) -> N + 'static + Clone + Send + Sync,
    KN: (Fn((usize, &T)) -> K) + 'static + Clone + Send + Sync,
    K: Eq + Hash + 'static,
    N: IntoView + 'static,
    S: With<Value = Vec<T>> + Copy + 'static + Send + Sync,
{
    let items_len_sig = Memo::new(move |_| each.with(|i| i.len()));
    let window_height = RwSignal::new(0);
    let window_width = RwSignal::new(0);

    let grid_items = Memo::new(move |_| {
        let window_width = window_width.get();
        (window_width / item_width).max(1)
    });

    let inner_height = Memo::new(move |_| {
        let grid_items = grid_items.get();
        if grid_items == 0 {
            return 0;
        }
        let items_len = items_len_sig.get();
        (items_len / grid_items) * item_height
    });

    let scroll_top = RwSignal::new(0);

    let index_bounds = Memo::new(move |_| {
        let scroll_top = scroll_top.get();
        let window_height = window_height.get();
        let items_len = items_len_sig.get();

        let grid_items_res = grid_items.get();

        let start_index_res = (scroll_top / item_height) * grid_items_res;
        let end_index_res =
            (((scroll_top + window_height) / item_height) * grid_items_res).min(items_len);

        (start_index_res, end_index_res)
    });

    let buffer_bounds = Memo::new(move |_| {
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

    let container = if let Some(node_ref) = node_ref {
        node_ref
    } else {
        NodeRef::new()
    };

    use_resize_observer(container, move |a, b| {
        let rect = a[0].content_rect();
        window_height.set(rect.height() as usize);
        window_width.set(rect.width() as usize);
    });

    let force_refresh = RwSignal::new(false);
    Effect::new(move || {
        each.with(|_| {});
        force_refresh.update(|v| {
            *v = !*v;
        });
    });

    view! {
        <div
            node_ref=container
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

            {
                move || {
                    let children = children.clone();
                    let key = key.clone();
                    force_refresh.get();

                    view !{
                        <For each=move || (buffer_bounds.get().0..buffer_bounds.get().1) key=move |i| {
                            each.with(|item| {
                                key((*i, item.get(*i).unwrap()))
                            })
                        } children=move |i| {
                            each.with(|item| {
                                let item = item.get(i).unwrap();
                                let (buffer_start, buffer_end) = buffer_bounds.get();
                                if i >= buffer_start && i <= buffer_end {
                                    let grid_index = i % grid_items.get();
                                    view! {
                                        <div
                                            style=format!("position: absolute; {}", inner_el_style)

                                            style:top=format!(
                                                "{}px",
                                                ((i) / grid_items.get()) * item_height,
                                            )
                                            style:left=format!("{}px", grid_index * item_width)
                                        >

                                            {children((i, item))}

                                        </div>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }
                            })
                        } />
                    }
                    }
            }

            </div>
        </div>
    }
}
