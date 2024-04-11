use leptos::html::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::error_modal;
use data::category::*;
use data::transaction::*;
use data::ResultWrapped;

async fn get_category_list() -> Vec<CategoryDisplay> {
    let ret_js: JsValue = super::invoke("get_category_display_list", JsValue::NULL).await;
    let ret: ResultWrapped<Vec<CategoryDisplay>, String> = from_value(ret_js).unwrap();
    return ret.res.unwrap();
}

#[component]
pub fn Categories() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    let categories = create_signal::<Vec<CategoryDisplay>>(vec![]);
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
            let ret_js: JsValue = super::invoke("create_category", args).await;
            let ret: ResultWrapped<(), String> = from_value(ret_js).unwrap();

            match ret.res {
                Err(v) => error_modal::show_error(v, &global_state),
                _ => {}
            }

            let cat_list = get_category_list().await;
            categories.1.set(cat_list);
        });
    };

    let (category_id_selected, category_id_selected_set) = create_signal(0);

    view! {
        <h1>
            "Categories"
        </h1>


            <div class="row">
            <div class="col-8">

                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th scope="col">Category</th>
                            <th scope="col">Activity</th>
                        </tr>
                    </thead>
                    <tbody>
                    {
                        move || {
                            categories.0.get().into_iter().map(
                            |val| {
                                view!{
                                    <tr on:click = move |ev| {
                                        category_id_selected_set.set(val.category_id);
                                        //log!("category selected {category_selected_id}");
                                    }>
                                        <td scope="row"
                                        class:highlight = move || category_id_selected.get() == val.category_id
                                        >
                                            {val.display_name}
                                        </td>

                                        <td
                                        class:highlight = move || category_id_selected.get() == val.category_id
                                        >
                                            {val.transaction_total}
                                        </td>
                                    </tr>
                                }

                            }
                            ).collect_view()
                        }
                    }
                    </tbody>
                </table>

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

            </div>
            <div class="col-4 bg-secondary-subtle rounded-3 p-3 px-4">
                <h3>"Categories Info"</h3>
                {
                    move || {
                        if category_id_selected.get() == 0 {
                            view! {
                                <p>"Select category to view detailed info."</p>
                            }
                        } else {
                            view! {
                                <p>"info here man"</p>
                            }
                        }
                    }
                }
            </div>
            </div>

    }
}
