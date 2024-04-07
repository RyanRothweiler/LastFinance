use leptos::html::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use data::account::AccountList;
use data::ResultWrapped;

async fn get_account_list() -> AccountList {
    let ret_js: JsValue = super::invoke("get_all_accounts", JsValue::NULL).await;
    let ret: ResultWrapped<AccountList, String> = from_value(ret_js).unwrap();
    return ret.res.unwrap();
}

#[component]
pub fn Nav() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    let accounts = create_signal::<AccountList>(AccountList::new());
    let account_res = create_resource(
        || (),
        move |_| async move {
            let list = get_account_list().await;
            accounts.1.set(list);
        },
    );

    let account_name_input: NodeRef<html::Input> = create_node_ref();

    let create_account = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            #[derive(Serialize, Deserialize)]
            struct Args<'a> {
                name: &'a str,
            }

            let args = to_value(&Args {
                name: &account_name_input.get().unwrap().value(),
            })
            .unwrap();
            let ret_js = super::invoke("create_account", args).await;
            let ret: ResultWrapped<(), String> = from_value(ret_js).unwrap();
            match ret.res {
                Err(v) => super::error_modal::show_error(v, &global_state),
                _ => {}
            }

            let account_list = get_account_list().await;
            accounts.1.set(account_list);
        });
    };

    let fund_account = move |account_id: i64| {
        spawn_local(async move {
            #[derive(Serialize, Deserialize)]
            struct Args {
                id: i64,
                cents: i64,
            }

            let args = to_value(&Args {
                id: account_id,
                cents: 1000,
            })
            .unwrap();
            let ret_js = super::invoke("fund_account", args).await;
            let ret: ResultWrapped<(), String> = from_value(ret_js).unwrap();
            match ret.res {
                Err(v) => super::error_modal::show_error(v, &global_state),
                _ => {}
            }

            let account_list = get_account_list().await;
            accounts.1.set(account_list);

            // update unassigned
            //unassigned_sig.update(|count: &mut f64| *count += 1.0 );
        });
    };

    view! {
        <nav class="col-md-2 d-none d-md-block sidebar px-4 py-2 bg-dark-subtle">

            <h3 class="text-secondary">Last Finance</h3>

            <div class="sidebar-sticky">
                <ul class="nav flex-column py-2">
                    <button class="nav-item gy-2" type="button" class="btn btn-dark" href="/">Budget</button>
                    <button class="nav-item gy-2" type="button" class="btn btn-dark" href="/transactions">Transactions</button>
                </ul>

                <h6 class="sidebar-heading text-muted justify-content-between aign-items-center">Accounts</h6>

                <ul class="nav flex-column">
                {
                move || {
                    accounts.0.get().accounts.into_iter().map(
                    |val| {
                        view!{
                            <li>
                                <h6>{val.display_name}</h6>
                                <p>{data::cents_to_dollars(val.balance)}$
                                <button type="submit" class="btn btn-outline-secondary btn-sm" on:click=move |_| {fund_account(val.id)}>fund</button>
                                </p>
                            </li>
                        }
                    }
                    ).collect_view()
                }
                }
                </ul>


                <ul class="nav flex-column">
                    <li class="nav-item">
                        <form on:submit=create_account>
                            <div class="form-group">
                                <input class="form-control" node_ref=account_name_input/>
                            </div>
                            <button type="submit" class="btn btn-outline-secondary btn-sm">Add Account</button>
                        </form>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
