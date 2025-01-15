use std::time::Duration;

use leptos::{prelude::*, task::spawn_local};
use leptos_virtual_scroller::{VirtualGridScroller, VirtualScroller};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[component]
fn App() -> impl IntoView {
    let mut items = vec![];
    for i in 0..1 {
        items.push(format!("hello {}", i));
    }

    let items_sig = RwSignal::new(items);
    let node_ref = NodeRef::new();

    let once = LocalResource::new(move || async move {
        set_timeout(
            move || {
                let mut items = vec![];
                for i in 1..10000 {
                    items.push(format!("hello {}", i));
                }
                items_sig.update(move |i| {
                    i.extend(items);
                    // i.shuffle(&mut thread_rng());
                });
            },
            Duration::from_secs(1),
        );
    });

    view! {
        <div style="height: 100vh;">
        {
            spawn_local(async move {
                once.await;
            });
        }
            <VirtualScroller
                node_ref=node_ref
                each=items_sig
                key=move|i| {
                    i.clone()
                }
                item_height=22
                header_height=200
                // item_width=200
                children=move |(index, item)| {
                    view! { <div>{item.clone()}</div> }
                }
                header=Some(move || {
                    view! {
                    <div style="background: blue; width: 200px; height: 200px;"></div>
                    }
                })
            />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! {
            <App />
        }
    })
}
