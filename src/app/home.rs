use leptos::html::*;
use leptos::leptos_dom::ev::{MouseEvent, SubmitEvent};
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::error_modal;
use data::account::*;
use data::ResultWrapped;

async fn get_account_list() -> AccountList {
    let ret_js: JsValue = super::invoke("get_all_accounts", JsValue::NULL).await;
    let ret: ResultWrapped<AccountList, String> = from_value(ret_js).unwrap();
    return ret.res.unwrap();
}

#[component]
pub fn Home() -> impl IntoView {
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
    let starting_balance_input: NodeRef<html::Input> = create_node_ref();

    let create_account = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            #[derive(Serialize, Deserialize)]
            struct Args<'a> {
                name: &'a str,
                sb: i64,
            }

            let name: &str = &account_name_input.get().unwrap().value();

            // parse starting balance
            let bal_str: &str = &starting_balance_input.get().unwrap().value();
            let starting_balance: f64 = match bal_str.parse::<f64>() {
                Ok(v) => v,
                Err(v) => {
                    error_modal::show_error(
                        "Error parsing starting balance".to_string(),
                        &global_state,
                    );
                    return;
                }
            };

            let args = to_value(&Args {
                name: name,
                sb: data::dollars_to_cents(starting_balance),
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
         <h1>
             "Home"
         </h1>

         <h3>
             "Accounts"
         </h3>

         <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#account_create">
             "Add Account"
         </button>

         <div class="modal fade" id="account_create" tabindex="-1" aria-labelledby="account_create" aria-hidden="true">
           <div class="modal-dialog modal-dialog-centered">
             <div class="modal-content">
               <div class="modal-header">
                 <h1 class="modal-title fs-5" id="exampleModalLabel">Create Account</h1>
                 <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
               </div>

               <div class="modal-body">
                 <form>

                    <div class="mb-3">
                        <label for="account_name" class="col-form-label">Account Name</label>
                        <input type="text" class="form-control" id="account_name" node_ref=account_name_input/>
                    </div>

                    <div class="mb-3">
                        <label for="starting_amount" class="col-form-label">Starting Balance</label>
                        <input type="number" class="form-control" id="starting_amount" node_ref=starting_balance_input/>
                    </div>

                </form>

               </div>

               <div class="modal-footer">
                 <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
                 <button type="submit" class="btn btn-primary" data-bs-dismiss="modal" on:click=create_account>
                 "Create"
                 </button>
               </div>
             </div>
           </div>
         </div>


         <ul class="nav flex-column">
         {
         move || {
             accounts.0.get().accounts.into_iter().map(
             |val| {
                 view!{
                     <li>
                         <h2>{val.display_name}</h2>
                         <p>{data::cents_to_dollars(val.balance)}</p>
                         <button type="submit" class="btn btn-outline-secondary btn-sm" on:click=move |_| {fund_account(val.id)}>"fund"</button>

                         <button class="btn btn-outline-secondary btn-sm" type="submit"
                         on:click = move |ev| {
                             spawn_local(async move {
                                 #[derive(Serialize, Deserialize)]
                                 struct Args {
                                     acc: i64,
                                 }
                                 let args = to_value(&Args {
                                     acc: val.id,
                                 })
                                 .unwrap();
                                 let ret_js = super::invoke("import", args).await;
                                 let ret: ResultWrapped<(), String> = from_value(ret_js).unwrap();
                                 match ret.res {
                                     Err(v) => {
                                         error_modal::show_error(v, &global_state);
                                     }
                                     Ok(()) => {}
                                 }

                             });
                         }
                         >
                             "Import CSV"
                         </button>

                    </li>
                 }
             }
             ).collect_view()
         }
         }
         </ul>

    }
}
