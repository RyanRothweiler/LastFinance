use leptos::html::*;
use leptos::leptos_dom::ev::{MouseEvent, SubmitEvent};
use leptos::logging::*;
use leptos::*;

use leptos_chartistry::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::error_modal;
use data::account::*;
use data::ResultWrapped;

use crate::app::invoke;
use crate::app::GlobalState;

use gloo_timers::future::TimeoutFuture;

async fn get_account_history(account_id: i64) -> Vec<AccountHistoryEntry> {
    #[derive(Serialize, Deserialize)]
    struct Args {
        acid: i64,
    }
    let args = to_value(&Args { acid: account_id }).unwrap();

    let ret_js: JsValue = invoke("get_account_history", args).await;
    let ret: ResultWrapped<Vec<AccountHistoryEntry>, String> = from_value(ret_js).unwrap();
    return ret.res.unwrap();
}

#[component]
pub fn AccountBox(account: AccountDisplay) -> impl IntoView {
    let global_state = expect_context::<RwSignal<GlobalState>>();

    let account_history = create_signal::<Vec<AccountHistoryEntry>>(vec![]);
    create_resource(
        || (),
        move |_| {
            let div_id = format!("graph_{0}", account.account_id);
            async move {
                let list = get_account_history(account.account_id).await;
                //account_history.1.set(list);

                TimeoutFuture::new(100).await;

                // Build data array. Can get this array directly from the database
                let mut data: Vec<f64> = vec![];
                for ent in list {
                    data.push(data::cents_to_dollars(ent.running_balance));
                }

                crate::app::js::build_graph(div_id, data);
            }
        },
    );

    // build graph data
    create_resource(
        || (),
        move |_| async move {
            TimeoutFuture::new(100).await;
        },
    );

    let graph_div_id: String = format!("graph_{0}", account.account_id);

    view! {
        <div class="col-md-6">
        <div class="bg-200 rounded-3 p-3 px-4 my-3">


        <div class="container-fluid">
          <div class="row">

            <div class="col">
                <h4>{account.display_name}</h4>
            </div>

            <div class="col text-end">
                <h1>{data::amount_to_display(account.balance)}</h1>
            </div>

          </div>
        </div>

        <div class="container-fluid">
            <div class="row">
                <div class="col-sm-2">
                </div>
                <div clss="col">
                </div>
            </div>
        </div>

        <div id={graph_div_id.clone()} style="width: max-width; height:400px;">
        </div>

         <button class="btn btn-outline-secondary btn-sm" type="submit"
         on:click = move |ev| {
             spawn_local(async move {
                 #[derive(Serialize, Deserialize)]
                 struct Args {
                     acc: i64,
                 }
                 let args = to_value(&Args {
                     acc: account.account_id,
                 })
                 .unwrap();
                 let ret_js = invoke("import", args).await;
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
             "Import Transactions CSV"
         </button>

         </div>
         </div>
    }
}
