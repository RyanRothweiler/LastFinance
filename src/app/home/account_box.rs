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
        move |_| async move {
            let list = get_account_history(account.account_id).await;
            account_history.1.set(list);
        },
    );

    view! {
        <div class="col-md-6">
        <div class="bg-secondary-subtle rounded-3 p-3 px-4 my-3">


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

        <Chart
            // Sets the width and height
            aspect_ratio=AspectRatio::from_outer_ratio(1000.0, 300.0)

            // Decorate our chart
            top = RotatedLabel::middle("Balance")
            left = TickLabels::aligned_floats()
            bottom = TickLabels::aligned_floats()
            inner = [
                AxisMarker::left_edge().into_inner(),
                AxisMarker::bottom_edge().into_inner(),
                XGridLine::default().into_inner(),
                YGridLine::default().into_inner(),
                XGuideLine::over_data().into_inner(),
                YGuideLine::over_mouse().into_inner(),
            ]
            tooltip = Tooltip::left_cursor()

            // Describe the data
            series = Series::new(|data: &AccountHistoryEntry| data.date as f64)
                .line(Line::new(|data: &AccountHistoryEntry| data::cents_to_dollars(data.running_balance)).with_name("balance"))
            data = account_history.0
        />


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
