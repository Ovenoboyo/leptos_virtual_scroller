use leptos::*;
use leptos_dom::logging::console_log;
use leptos_virtual_scroller::{VirtualGridScroller, VirtualScroller};

fn main() {
    console_error_panic_hook::set_once();

    let mut items = vec![];
    for i in 0..10000 {
        items.push(format!("hello {}", i));
    }

    let items_sig = create_rw_signal(items);
    let node_ref = create_node_ref();

    mount_to_body(move || {
        view! {
            <div style="height: 100vh;">
                <VirtualScroller
                    node_ref=node_ref
                    each=items_sig
                    key=move|i| {
                        i.clone()
                    }
                    item_height=200
                    header_height=300
                    // item_width=200
                    children=move |(index, item)| {
                        console_log(&format!("rerendered {}", index));
                        view! { <div>{item}</div> }
                    }
                    header=Some(move || {
                        view! {
                            <div style="height: 300px; width: 300px; background: blue;"></div>
                        }
                    })
                />
            </div>
        }
    })
}
