use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_wasm_bindgen::to_value;


#[component]
pub fn Nav() -> impl IntoView {
    let create_account = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            #[derive(Serialize, Deserialize)]
            struct Args<'a> {
                name: &'a str,
            }

            let args = to_value(&Args { name: "new account" }).unwrap();
            super::invoke("create_account", args).await;
            log!("createing account");

            //let cat_list = get_category_list().await;
            //categories.1.set(cat_list);
        });
    };

    view! {
        <nav class="col-md-2 d-none d-md-block sidebar">
            <div class="sidebar-sticky">
                <ul class="nav flex-column">
                    <li class="nav-item">
                        <a class="nav-link">Bar Here</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link">Bar Here</a>
                    </li>

                </ul>
                <h6 class="sidebar-heading text-muted justify-content-between aign-items-center">Accounts</h6>

                <ul class="nav flex-column">
                    <li class="nav-item">
                        <form on:submit=create_account>
                            <div class="form-group">
                                <input class="form-control"/>
                            </div>
                        <button type="submit" class="btn btn-outline-secondary btn-sm">Add Account</button>
                        </form>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
