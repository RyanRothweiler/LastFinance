use leptos::html::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::error_modal;
use data::transaction::*;
use data::ResultWrapped;

async fn get_transactions_list() -> TransactionDisplayList {
    let ret_js: JsValue = super::invoke("get_all_transactions_display", JsValue::NULL).await;
    let ret: ResultWrapped<TransactionDisplayList, String> = from_value(ret_js).unwrap();
    return ret.res.unwrap();
}

#[component]
pub fn Transactions() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    let transactions = create_signal::<TransactionDisplayList>(TransactionDisplayList::new());
    create_resource(
        || (),
        move |_| async move {
            let lst = get_transactions_list().await;
            transactions.1.set(lst);
        },
    );

    let create_transaction_payee_nr: NodeRef<html::Input> = create_node_ref();
    let create_transaction_date_nr: NodeRef<html::Input> = create_node_ref();
    let create_transaction_category_nr: NodeRef<html::Input> = create_node_ref();

    let (outflow_get, outflow_set) = create_signal("".to_string());
    let (inflow_get, inflow_set) = create_signal("".to_string());

    let create_transaction = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            // parse amount
            let outflow: i64 = match outflow_get.get().parse::<i64>() {
                Ok(v) => v,
                Err(v) => 0,
            };

            // parse amount
            let inflow: i64 = match inflow_get.get().parse::<i64>() {
                Ok(v) => v,
                Err(v) => 0,
            };

            let mut trans = match Transaction::new(
                create_transaction_payee_nr.get_untracked().unwrap().value(),
                inflow,
                outflow,
                100,
                0,
            ) {
                Ok(v) => v,
                Err(v) => {
                    error_modal::show_error(v, &global_state);
                    return;
                }
            };

            // get category id from name
            {
                #[derive(Serialize, Deserialize)]
                struct CategoryArgs<'a> {
                    name: &'a str,
                }
                let args = to_value(&CategoryArgs {
                    name: &create_transaction_category_nr.get().unwrap().value(),
                })
                .unwrap();
                let ret_js: JsValue = super::invoke("get_category_id", args).await;
                let ret: ResultWrapped<i64, String> = from_value(ret_js).unwrap();
                match ret.res {
                    Ok(v) => trans.category_id = v,
                    Err(v) => {
                        error_modal::show_error(
                            "No category with that name.".to_string(),
                            &global_state,
                        );
                        return;
                    }
                }
            }

            #[derive(Serialize, Deserialize)]
            struct Args {
                trans: Transaction,
            }

            let args = to_value(&Args { trans: trans }).unwrap();

            let ret_js: JsValue = super::invoke("create_transaction", args).await;
            let ret: ResultWrapped<(), String> = from_value(ret_js).unwrap();
            match ret.res {
                Err(v) => error_modal::show_error(format!("{:?}", v), &global_state),
                _ => {}
            }

            let lst = get_transactions_list().await;
            transactions.1.set(lst);
        });
    };

    view! {
        <h1>
            Transactions
        </h1>
        <table class="table table-sm">
            <thead>
                <tr>
                    <th scope="col">Payee</th>
                    <th scope="col">Category</th>
                    <th scope="col">Outflow</th>
                    <th scope="col">Inflow</th>
                </tr>
            </thead>
            <tbody>
            {
                move || {
                    transactions.0.get().transactions.into_iter().map(
                    |val| {
                        let mut outflow: String = "".to_string();
                        let mut inflow: String = "".to_string();
                        if val.trans_raw.amount > 0 {
                            inflow = format!("{:?}", val.trans_raw.amount);
                        } else {
                            outflow = format!("{:?}", -val.trans_raw.amount);
                        }

                        view!{
                            <tr>
                                <td style="width:50%">{val.trans_raw.payee}</td>
                                <td>{val.category_display}</td>
                                <td style="width:5%">{outflow}</td>
                                <td style="width:5%">{inflow}</td>
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
                <input class="form-control" placeholder="Outflow" type="number"
                    on:input=move |ev| {
                        outflow_set.set(event_target_value(&ev));
                        inflow_set.set("".to_string());
                    }
                    prop:value = outflow_get
                />
            </div>

            <div class="col-12">
                <input class="form-control" placeholder="Inflow" type="number"
                    on:input=move |ev| {
                        inflow_set.set(event_target_value(&ev));
                        outflow_set.set("".to_string());
                    }
                    prop:value=inflow_get
                />
            </div>

            <div class="col-12">
                <input class="form-control" placeholder="Category" node_ref=create_transaction_category_nr/>
            </div>

            <div class="col-12">
            <button class="btn btn-primary" type="submit">"Add Transaction"</button>
            </div>

        </form>
    }
}
