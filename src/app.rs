#![allow(unused_variables, unused_imports, dead_code)]

mod error_modal;
mod nav;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_wasm_bindgen::to_value;

use wasm_bindgen::prelude::*;

use data::account::Account;
use data::account::AccountList;
use data::category::Category;
use data::category::CategoryList;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

mod js {

    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/public/last_finance.js")]
    extern "C" {
        pub fn show_error() -> JsValue;
    }
}

#[derive(Clone, Debug, Default)]
struct GlobalState {
    error: String,
}

async fn get_category_list() -> CategoryList {
    let json = invoke("get_all_categories", JsValue::NULL)
        .await
        .as_string()
        .unwrap();
    let list: CategoryList = serde_json::from_str(&json).unwrap();
    return list;
}

async fn get_account_list() -> AccountList {
    let json = invoke("get_all_accounts", JsValue::NULL)
        .await
        .as_string()
        .unwrap();
    let list: AccountList = serde_json::from_str(&json).unwrap();
    return list;
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(GlobalState::default()));

    let categories = create_signal::<CategoryList>(CategoryList::new());
    let category_res = create_resource(
        || (),
        move |_| async move {
            let cat_list = get_category_list().await;
            categories.1.set(cat_list);
        },
    );

    let accounts = create_signal::<AccountList>(AccountList::new());
    let account_res = create_resource(
        || (),
        move |_| async move {
            let list = get_account_list().await;
            accounts.1.set(list);
        },
    );

    let (name, set_name) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            #[derive(Serialize, Deserialize)]
            struct Args<'a> {
                name: &'a str,
            }

            let args = to_value(&Args { name: &name }).unwrap();
            invoke("create_category", args).await;

            let cat_list = get_category_list().await;
            categories.1.set(cat_list);
        });
    };

    let input_element: NodeRef<html::Input> = create_node_ref();
    let submit_account = move |ev: SubmitEvent| {
        ev.prevent_default();
        let val = input_element.get().expect("heyo").value();
    };

    view! {
        <html data-bs-theme="dark">
        <body>
        <main>
            <div class="container-fluid">

                <div class="row">
                    <nav::Nav/>
                    <div class="col-md-9">

                        <ul>
                        {
                        move || {
                            accounts.0.get().accounts.into_iter().map(
                            |val| {
                                view!{<li>{val.balance}</li>}
                            }
                            ).collect_view()
                        }
                        }
                        </ul>

                        <ul>
                        {
                        move || {
                            categories.0.get().categories.into_iter().map(
                            |val| {
                                view!{<li>{val.display_name}</li>}
                            }
                            ).collect_view()
                        }
                        }
                        </ul>

                        <div class="col">
                            <form class="row" on:submit=greet>
                                <input
                                    id="greet-input"
                                    placeholder="Enter a name..."
                                    on:input=update_name
                                />
                                <button class="btn btn-primary" type="submit">"Add Category"</button>
                            </form>
                        </div>


                        <div class="col">
                            <form class="row" on:submit=submit_account>
                            <input type="text"
                                placeholder="Enter a name..."
                                node_ref=input_element
                                />

                                <button class="btn btn-primary" type="submit">"Add Account"</button>

                            </form>
                        </div>


                        <div class="dropdown" data-bs-theme="dark">
                          <button class="btn btn-secondary dropdown-toggle" type="button" id="dropdownMenuButtonDark" data-bs-toggle="dropdown" aria-expanded="false">
                            Dark dropdown
                          </button>
                          <ul class="dropdown-menu" aria-labelledby="dropdownMenuButtonDark">
                            <li><a class="dropdown-item active" href="#">Action</a></li>
                            <li><a class="dropdown-item" href="#">Action</a></li>
                          </ul>
                        </div>

                        <error_modal::ErrorModal/>

                    </div>
                </div>


            </div>

        </main>
        </body>
        </html>
    }
}
