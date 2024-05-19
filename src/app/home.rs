use leptos::html::*;
use leptos::leptos_dom::ev::{MouseEvent, SubmitEvent};
use leptos::logging::*;
use leptos::*;

use leptos_chartistry::*;
use tauri_sys::tauri;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::error_modal;
use data::account::*;
use data::ResultWrapped;
use data::RytError;

mod account_box;

async fn get_account_list() -> Vec<AccountDisplay> {
    let ret_js: JsValue = super::invoke("get_account_display_list", JsValue::NULL).await;
    let ret: ResultWrapped<Vec<AccountDisplay>, String> = from_value(ret_js).unwrap();
    return ret.res.unwrap();
}

#[component]
pub fn Home() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    let accounts = create_signal::<Vec<AccountDisplay>>(vec![]);
    create_resource(
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

            // invoke
            let res = tauri::invoke(
                "create_account",
                &Args {
                    name: name,
                    sb: data::dollars_to_cents(starting_balance),
                },
            )
            .await;
            let ret: Result<i64, RytError> = super::convert_invoke(res);
            match ret {
                Err(v) => {
                    super::error_modal::show_error(v.to_string(), &global_state);
                }
                _ => {}
            }

            let account_list = get_account_list().await;
            accounts.1.set(account_list);
        });
    };

    view! {
        <div class="container-fluid">


          <div class="row">

            <div class="col">
                <h1>
                    "Accounts"
                </h1>
            </div>
            <div class="col text-end">
                <button type="button" class="btn btn-secondary" data-bs-toggle="modal" data-bs-target="#account_create">

                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-plus-circle" viewBox="0 0 16 16">
                      <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"/>
                      <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
                    </svg>
                    "Add Account"
                </button>



            </div>

          </div>
        </div>

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


         <div class="container-fluid">
         <div class="row">

         {
            move || {
                if accounts.0.get().len() == 0 {
                    return view! {
                        <h1>"Add an account to get started."</h1>
                    };
                }

                return view!{<h1></h1>};
            }
         }

         {
         move || {
             accounts.0.get().into_iter().map(
             |val| {
                 view!{
                     <account_box::AccountBox account=val/>
                }
             }
             ).collect_view()
         }
         }

        </div>
        </div>

    }
}
