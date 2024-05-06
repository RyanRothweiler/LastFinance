use leptos::html::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

#[component]
pub fn Nav() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    let (db_name_get, db_name_set) = create_signal::<String>("".to_string());
    create_resource(
        || (),
        move |_| async move {
            db_name_set.set("heyo".to_string());
        },
    );

    view! {
        <div class="side_nav">
            <p class="side_nav_button"><a href="/">Overview</a></p>
            <p class="side_nav_button"><a href="/categories">Categories</a></p>
            <p class="side_nav_button"><a href="/transactions">Transactions</a></p>
        </div>
    }
}
