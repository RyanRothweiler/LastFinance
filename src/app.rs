#![allow(unused_variables, unused_imports)]

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_wasm_bindgen::to_value;

use wasm_bindgen::prelude::*;

use data::category::Category;
use data::category::CategoryList;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_category_list() -> CategoryList {
    let json = invoke("get_all_categories", JsValue::NULL)
        .await
        .as_string()
        .unwrap();
    let list: CategoryList = serde_json::from_str(&json).unwrap();
    return list;
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let categories = create_signal::<CategoryList>(CategoryList::new());

    let category_init = create_resource(
        || (),
        move |_| async move {
            let cat_list = get_category_list().await;
            categories.1.set(cat_list);
        },
    );

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

    view! {
        <main class="container">
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

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Add Category"</button>
            </form>
        </main>
    }
}
