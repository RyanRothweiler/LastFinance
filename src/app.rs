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
use data::transaction::*;

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

async fn get_transactions_list() -> TransactionDisplayList {
    let json = invoke("get_all_transactions_display", JsValue::NULL)
        .await
        .as_string()
        .unwrap();
    let list: TransactionDisplayList = serde_json::from_str(&json).unwrap();
    return list;
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

    let transactions = create_signal::<TransactionDisplayList>(TransactionDisplayList::new());
    create_resource(
        || (),
        move |_| async move {
            let lst = get_transactions_list().await;
            transactions.1.set(lst);
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
            let json = invoke("create_category", args).await.as_string().unwrap();

            let res: Result<(), String> = serde_json::from_str(&json).unwrap();
            match res {
                Err(v) => {
                    error_modal::show_error(format!("{:?}", v), &global_state);
                    log!("error!");
                }
                _ => {}
            }

            let cat_list = get_category_list().await;
            categories.1.set(cat_list);
        });
    };

    let create_transaction_payee_nr: NodeRef<html::Input> = create_node_ref();
    let create_transaction_amount_nr: NodeRef<html::Input> = create_node_ref();
    let create_transaction_date_nr: NodeRef<html::Input> = create_node_ref();
    let create_transaction = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            // parse amount
            let amount: i64 = match create_transaction_amount_nr
                .get()
                .unwrap()
                .value()
                .parse::<i64>()
            {
                Ok(v) => v,
                Err(v) => {
                    error_modal::show_error(format!("Error parsing input. {:?}", v), &global_state);
                    return;
                }
            };

            #[derive(Serialize, Deserialize)]
            struct Args {
                trans: Transaction,
            }

            let args = to_value(&Args {
                trans: Transaction::new(
                    create_transaction_payee_nr.get_untracked().unwrap().value(),
                    amount,
                    100,
                    0,
                ),
            })
            .unwrap();

            let json = invoke("create_transaction", args)
                .await
                .as_string()
                .unwrap();

            let res: Result<(), String> = serde_json::from_str(&json).unwrap();
            match res {
                Err(v) => {
                    error_modal::show_error(format!("{:?}", v), &global_state);
                    log!("error!");
                }
                _ => {}
            }

            let lst = get_transactions_list().await;
            transactions.1.set(lst);
        });
    };

    view! {
        <html data-bs-theme="dark">
        <body>
        <main>
            <div class="container-fluid">

                <div class="row">
                    <nav::Nav unassigned_sig=unassigned_set/>

                    <div class="col-md-10">

                        <h1>
                            Unassigned {unassigned}
                        </h1>

                        <h1>
                            Transactions
                        </h1>
                        <table class="table table-sm">
                            <thead>
                                <tr>
                                    <th scope="col">Payee</th>
                                    <th scope="col">Category</th>
                                    <th scope="col">Amount</th>
                                </tr>
                            </thead>
                            <tbody>
                            {
                                move || {
                                    transactions.0.get().transactions.into_iter().map(
                                    |val| {
                                        view!{
                                            <tr>
                                                <th scope="row" style="width:50%">{val.trans_raw.payee}</th>
                                                <td>{val.category_display}</td>
                                                <td>{val.trans_raw.amount}</td>
                                            </tr>
                                        }
                                    }
                                    ).collect_view()
                                }
                            }
                            </tbody>
                        </table>

                        <form class="row row-cols-lg-auto" on:submit=create_transaction>
                            <div class="col-12">
                                <input class="form-control" placeholder="Payee" node_ref=create_transaction_payee_nr/>
                            </div>

                            <div class="col-12">
                                <input class="form-control" placeholder="Date" type="date" node_ref=create_transaction_date_nr/>
                            </div>

                            <div class="col-12">
                                <input class="form-control" placeholder="Amount" type="number" node_ref=create_transaction_amount_nr/>
                            </div>

                            <div class="col-12">
                                <input class="form-control" placeholder="Category"/>
                            </div>

                            <div class="col-12">
                            <button class="btn btn-primary" type="submit">"Add Transaction"</button>
                            </div>

                        </form>

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

        </main>
        </body>
        </html>
    }
}
