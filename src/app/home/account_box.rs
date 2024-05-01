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

        <style>"
            .my-theme {
                /* Use 'fill' for filling text colour */
                fill: #ddd;

                /* Some elements (e.g., legend and tooltips) use HTML so we
                    still still need to set 'color' */
                color: #ddd;
            }

            /* We can set stroke (and fill) directly too */
            .my-theme ._chartistry_grid_line_x {
                stroke: #505050;
            }

            /*data::cents_to_dollars(data.running_balance) * 0.001 The tooltip uses inline CSS styles and so must be overridden */
            .my-theme ._chartistry_tooltip {
                border: 1px solid #fff !important;
                background-color: #333 !important;
            }

        "</style>

        <div class="my-theme">
        <Chart
            // Sets the width and height
            aspect_ratio = AspectRatio::from_env_width_apply_ratio(3.0)

            // Decorate our chart
            top = RotatedLabel::middle("Balance")
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
                .line(Line::new(|data: &AccountHistoryEntry| {
                    let d = data::cents_to_dollars(data.running_balance) * 0.001;
                    log!("testing {d}");
                    return data::cents_to_dollars(data.running_balance) * 0.001;
        }).with_name("balance").with_width(2.0))
            data = account_history.0
        />
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
