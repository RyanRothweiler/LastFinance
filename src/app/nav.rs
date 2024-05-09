use leptos::html::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use data::{DatabaseInfo, ResultWrapped};

#[component]
pub fn Nav() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    let (db_info_get, db_info_set) = create_signal::<DatabaseInfo>(DatabaseInfo {
        file_name: "".to_string(),
        file_path: "".to_string(),
    });
    create_resource(
        || (),
        move |_| async move {
            let ret_js: JsValue = super::invoke("get_db_info", JsValue::NULL).await;
            // TODO error modal here
            let db_info: ResultWrapped<DatabaseInfo, String> = from_value(ret_js).unwrap();
            db_info_set.set(db_info.res.unwrap());
        },
    );

    view! {
        <div class="side_nav">
            <h3>"Last Finance"</h3>

            <hr></hr>

            <p class="side_nav_button"><a href="/">Overview</a></p>
            <p class="side_nav_button"><a href="/categories">Categories</a></p>
            <p class="side_nav_button"><a href="/transactions">Transactions</a></p>

            <hr></hr>

            <p class="text-body-secondary current_db_p"><strong>"Current Open Database"</strong></p>
            <p class="text-body-secondary current_db_p">
            {
                move || db_info_get.get().file_name
            }
            </p>
            <p class="text-body-secondary current_db_p fs-6">
            {
                move || db_info_get.get().file_path
            }
            </p>

            <button class="btn btn-outline-secondary btn-sm">Open New Database</button>
            <button class="btn btn-outline-secondary btn-sm"
                on:click = move |ev| {
                    spawn_local(async move {
                        let ret_js: JsValue = super::invoke("create_db", JsValue::NULL).await;
                        super::js::reload_page();

                        // TODO handle error
                        //let db_info: ResultWrapped<(), String> = from_value(ret_js).unwrap();
                        //db_info_set.set(db_info.res.unwrap());
                    });
                }
            >
                Create New Database
            </button>

            <hr></hr>

        </div>
    }
}
