#![allow(unused_variables, unused_imports, dead_code, unused_assignments)]

mod categories;
mod error_modal;
mod home;
mod nav;
mod transactions;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;
use leptos_router::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use wasm_bindgen::prelude::*;

use data::account::*;
use data::category::Category;
use data::category::CategoryList;
use data::ResultWrapped;

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
        pub fn build_graph(element_id: String, data: Vec<f64>);
        pub fn reload_page();
    }
}

#[derive(Clone, Debug, Default)]
struct GlobalState {
    error: String,
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(GlobalState::default()));
    let global_state = expect_context::<RwSignal<GlobalState>>();

    view! {
        <html data-bs-theme="dark">
        <main>

        <Router>

            <body>
            <div class="flex_holder app_container">
                <nav::Nav/>

                <div class="page_content">
                    <div class="p-4">
                        <Routes>
                            <Route path="/" view=home::Home/>
                            <Route path="/transactions" view=transactions::Transactions/>
                            <Route path="/categories" view=categories::Categories/>
                        </Routes>

                       <error_modal::ErrorModal/>

                    </div>
                </div>
            </div>
            </body>

        </Router>
        </main>
        </html>
    }
}
