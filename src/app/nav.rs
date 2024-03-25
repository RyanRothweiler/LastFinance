use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_wasm_bindgen::to_value;

use data::account::AccountList;

async fn get_account_list() -> AccountList {
    let json = super::invoke("get_all_accounts", JsValue::NULL)
        .await
        .as_string()
        .unwrap();
    let list: AccountList = serde_json::from_str(&json).unwrap();
    return list;
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
            let json = super::invoke("create_account", args)
                .await
                .as_string()
                .unwrap();

            let res: Result<(), String> = serde_json::from_str(&json).unwrap();
            match res {
                Err(v) => super::error_modal::show_error(v, &global_state),
                _ => {}
            }

            let account_list = get_account_list().await;
            accounts.1.set(account_list);
        });
    };

    view! {
        <nav class="col-md-2 d-none d-md-block sidebar">
            <div class="sidebar-sticky">
                <ul class="nav flex-column">
                    <li class="nav-item">
                        <a class="nav-link">Bar Here</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link">Bar Here</a>
                    </li>

                </ul>
                <h6 class="sidebar-heading text-muted justify-content-between aign-items-center">Accounts</h6>

                <ul class="nav flex-column">
                {
                move || {
                    accounts.0.get().accounts.into_iter().map(
                    |val| {
                        view!{<li>{val.display_name} -> {val.balance}</li>}
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
