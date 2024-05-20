use leptos::html::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use tauri_sys::tauri;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use data::{RytError, DatabaseInfo};

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
            let res = tauri::invoke("get_db_info", &crate::app::NoArgs {}).await;
            let ret: Result<DatabaseInfo, RytError> = crate::app::convert_invoke(res);

            // TODO handle error
            db_info_set.set(ret.unwrap());
        },
    );

    let create_db = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let res = tauri::invoke("create_db", &crate::app::NoArgs {}).await;
            let ret: Result<(), RytError> = crate::app::convert_invoke(res);

            // TODO handle error
            super::js::reload_page();
        });
    };

    let open_db = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let res = tauri::invoke("open_db", &crate::app::NoArgs {}).await;
            let ret: Result<(), RytError> = crate::app::convert_invoke(res);

            // TODO handle error
            super::js::reload_page();
        });
    };

    let export_csv = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let res = tauri::invoke("export_to_csv", &crate::app::NoArgs {}).await;
            let ret: Result<(), RytError> = crate::app::convert_invoke(res);

            // TODO handle error
        });
    };

    view! {
        <div class="side_nav">
            <h3>"Last Finance"</h3>

            <hr></hr>

            <p class="side_nav_button"><a href="/">Overview</a></p>
            <p class="side_nav_button"><a href="/categories">Categories</a></p>
            <p class="side_nav_button"><a href="/transactions">Transactions</a></p>


            <div class="side_nav_align_bottom">
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
                <p></p>
                <div class="d-grid gap-2">
                    <button class="btn btn-secondary btn-sm" type="button"
                        on:click = open_db
                    >
                    "Open Existing Database"
                    </button>

                    <button class="btn btn-secondary btn-sm" type="button"
                        on:click = create_db
                    >
                    "Create New Database"
                    </button>
                    <button class="btn btn-outline-secondary btn-sm" type="button"
                        on:click = export_csv
                    >
                    "Export Database to CSV"
                    </button>

                </div>
            </div>

        </div>
    }
}
