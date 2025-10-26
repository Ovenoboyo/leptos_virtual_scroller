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
                for i in 1..200 {
                    items.push(format!("hello {}", i));
                }
                items_sig.update(move |i| {
                    i.extend(items);
                    i.shuffle(&mut thread_rng());
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
                key=move|(_index, item)| {
                    Some(item.clone())
                }
                item_height=275
                header=()
                // item_width=220
                children=move |(_index, item)| {
                    view! {
                        <div>
                          <div style="width: 200px; margin-bottom: 0.5rem; transition: transform 0.3s ease-in-out;">
                            <div style="width: 100%; position: relative; padding-top: 100%; overflow: hidden;">
                              <div style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;">
                                <div style="position: absolute; width: 100%; height: 100%; background-color: rgba(0,0,0,0.5); z-index: 1;"></div>
                                <div style="position: absolute; width: 100%; height: 100%; display: flex; justify-content: center; align-items: center;">
                                </div>
                                <svg style="border-radius: 8px; width: 100%; height: 100%;" viewBox="0 0 201 201" fill="none" xmlns="http://www.w3.org/2000/svg">
                                  <rect width="201" height="201" style="fill: var(--secondary);"></rect>
                                  <path d="M146.899 46.3652C146.717 45.2724 146.303 44.2314 145.684 43.3122C145.066 42.3931 144.257 41.6173 143.314 41.037C142.37 40.4568 141.313 40.0857 140.213 39.9487C139.114 39.8116 137.998 39.9119 136.941 40.2427L82.6905 57.1997C81.1131 57.6922 79.7343 58.676 78.7556 60.0075C77.7768 61.339 77.2493 62.9485 77.25 64.601V127.624C73.7583 125.005 69.4847 123.642 65.1216 123.756C60.7585 123.869 56.5615 125.452 53.2105 128.249C49.8596 131.045 47.5511 134.892 46.659 139.164C45.767 143.436 46.3436 147.885 48.2955 151.789C50.2474 155.693 53.4602 158.823 57.4134 160.673C61.3667 162.523 65.8286 162.983 70.0765 161.981C74.3243 160.978 78.1091 158.57 80.8177 155.148C83.5263 151.725 85 147.489 85 143.124V87.851L139.25 70.894V112.124C135.758 109.505 131.485 108.142 127.122 108.256C122.758 108.369 118.561 109.952 115.211 112.749C111.86 115.545 109.551 119.392 108.659 123.664C107.767 127.936 108.344 132.385 110.295 136.289C112.247 140.193 115.46 143.323 119.413 145.173C123.367 147.023 127.829 147.483 132.076 146.481C136.324 145.478 140.109 143.07 142.818 139.648C145.526 136.225 147 131.989 147 127.624V47.644C147 47.21 146.961 46.7837 146.899 46.3652Z" fill="var(--textSecondary)"></path>
                                </svg>
                              </div>
                            </div>
                            <div style="padding: 0.75rem;">
                              <p style="margin: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; font-weight: bold;">{item.to_string()}</p>
                            </div>
                          </div>
                        </div>
                    }
                }
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
