use leptos::*;
use leptos_virtual_scroller::VirtualGridScroller;

fn main() {
    console_error_panic_hook::set_once();

    let mut items = vec![];
    for i in 0..10000 {
        items.push(format!("hello {}", i));
    }

    let items_sig = create_rw_signal(items);

    mount_to_body(move || {
        view! {
            <div style="height: 100vh;">
                <VirtualGridScroller
                    each=items_sig
                    item_height=200
                    item_width=200
                    children=move |(index, item)| {
                        view! { <div>{item}</div> }
                    }
                />
            </div>
        }
    })
}
