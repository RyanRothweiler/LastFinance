#![allow(unused_variables, unused_imports, dead_code, unused_assignments)]

mod error_modal;
mod nav;
mod transactions;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;
use leptos_router::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use wasm_bindgen::prelude::*;

use data::account::Account;
use data::account::AccountList;
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
    }
}

#[derive(Clone, Debug, Default)]
struct GlobalState {
    error: String,
}

async fn get_category_list() -> CategoryList {
    let ret_js: JsValue = invoke("get_all_categories", JsValue::NULL).await;
    let ret: ResultWrapped<CategoryList, String> = from_value(ret_js).unwrap();
    return ret.res.unwrap();
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(GlobalState::default()));
    let global_state = expect_context::<RwSignal<GlobalState>>();

    let categories = create_signal::<CategoryList>(CategoryList::new());
    create_resource(
        || (),
        move |_| async move {
            let lst = get_category_list().await;
            categories.1.set(lst);
        },
    );

    let (unassigned, unassigned_set) = create_signal(100.0);

    let (name, set_name) = create_signal(String::new());
    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let create_category = move |ev: SubmitEvent| {
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
            let ret_js: JsValue = invoke("create_category", args).await;
            let ret: ResultWrapped<(), String> = from_value(ret_js).unwrap();

            match ret.res {
                Err(v) => error_modal::show_error(v, &global_state),
                _ => {}
            }

            let cat_list = get_category_list().await;
            categories.1.set(cat_list);
        });
    };

    view! {
        <html data-bs-theme="dark">
        <body>
        <main>
        <Router>
            <div class="container-fluid">

                <div class="row">
                    <nav::Nav unassigned_sig=unassigned_set/>


                    <div class="col-md-10">
                        <Routes>
                            <Route path="/" view=|| view!{ <h1>"home"</h1>}/>
                            <Route path="/transactions" view=transactions::Transactions/>
                        </Routes>

                        <h1>
                            Unassigned {unassigned}
                        </h1>

                       <h1>
                            Categories
                        </h1>

                        {
                            move || {
                                categories.0.get().categories.into_iter().map(
                                |val| {
                                    view!{<li>{val.display_name}</li>}
                                }
                                ).collect_view()
                            }
                        }

                        <form class="row row-cols-lg-auto" on:submit=create_category>
                            <div class="col-12">
                            <input
                                class="form-control"
                                placeholder="Enter a name..."
                                on:input=update_name
                            />
                            </div>

                            <div class="col-12">
                            <button class="btn btn-primary" type="submit">"Add Category"</button>
                            </div>

                        </form>


                        <div class="dropdown" data-bs-theme="dark">
                          <ul class="dropdown-menu" aria-labelledby="dropdownMenuButtonDark">
                            <li><a class="dropdown-item active" href="#">Action</a></li>
                            <li><a class="dropdown-item" href="#">Action</a></li>
                          </ul>
                        </div>

                        <error_modal::ErrorModal/>

                    </div>
                </div>


            </div>

        </Router>
        </main>
        </body>
        </html>
    }
}
