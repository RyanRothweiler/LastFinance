use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{to_value, from_value};

use super::GlobalState;

pub fn show_error(display: String, global_state: &RwSignal<GlobalState>) {
    super::js::show_error();
    log!("Error \n {}", display);

    global_state.update(|v: &mut GlobalState| {
        v.error = display;
    });
}

#[component]
pub fn ErrorModal() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    view! {
    <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
        <div class="modal-dialog modal-lg">
        <div class="modal-content">
          <div class="modal-header">
                <h1 class="modal-title fs-5 text-danger" id="exampleModalLabel">Error</h1>
          </div>
          <div class="modal-body">
                <p>{move || global_state.get().error}</p>
                <p class="text-muted">Contact info here</p>
          </div>
          <div class="modal-footer">
            <div class="container">
                <div class="row">
                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
                </div>
            </div>
          </div>
        </div>
      </div>
    </div>


        }
}
