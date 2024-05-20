use leptos::html::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

use tauri_sys::tauri;
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::error_modal;
use data::category::*;
use data::transaction::*;
use data::RytError;

use chrono::prelude::*;

async fn get_category_list(year: i32, month: u32) -> Vec<CategoryDisplay> {
    let mut date_start = chrono::Utc
        .with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .unwrap();

    let mut date_end = date_start
        .checked_add_months(chrono::Months::new(1))
        .unwrap();

    // Shift back one second.
    // We want to get date from previous month 23::59::59 to current month 23::59::59
    // Otherwise we'll get transactions right on the edges.
    // This assumes no transactions happen exactly a 23::59::59
    date_start = date_start
        .checked_sub_signed(chrono::TimeDelta::seconds(1))
        .unwrap();
    date_end = date_end
        .checked_sub_signed(chrono::TimeDelta::seconds(1))
        .unwrap();

    #[derive(Serialize, Deserialize)]
    struct Args {
        start: i64,
        end: i64,
    }

    let start = date_start.timestamp();
    let end = date_end.timestamp();

    let args = to_value(&Args {
        start: start,
        end: end,
    })
    .unwrap();

    let res = tauri::invoke(
        "get_category_display_list",
        &Args {
            start: start,
            end: end,
        },
    )
    .await;
    let ret: Result<Vec<CategoryDisplay>, RytError> = super::convert_invoke(res);

    // TODO handle error
    return ret.unwrap();
}

#[component]
pub fn Categories() -> impl IntoView {
    let global_state = expect_context::<RwSignal<super::GlobalState>>();

    let (month_selected, month_selected_set) = create_signal::<u32>(1);
    let (year_selected, year_selected_set) = create_signal::<i32>(2024);

    let categories = create_signal::<Vec<CategoryDisplay>>(vec![]);
    create_resource(
        || (),
        move |_| async move {
            let date = chrono::Utc
                .with_ymd_and_hms(
                    year_selected.get_untracked(),
                    month_selected.get_untracked(),
                    1,
                    0,
                    0,
                    0,
                )
                .unwrap();

            let lst = get_category_list(
                year_selected.get_untracked(),
                month_selected.get_untracked(),
            )
            .await;
            categories.1.set(lst);
        },
    );

    let (unassigned, unassigned_set) = create_signal(100.0);

    let (name, set_name) = create_signal(String::new());
    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let create_category = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            #[derive(Serialize, Deserialize)]
            struct Args<'a> {
                name: &'a str,
            }

            let res = tauri::invoke("create_category", &Args { name: &name }).await;

            // TODO handle error
            let ret: Result<i64, RytError> = super::convert_invoke(res);
        });
    };

    let (category_id_selected, category_id_selected_set) = create_signal(0);

    view! {
        <div class="btn-group" role="group">
            <button type="button" class="button_icon_hidden"
                on:click = move |ev| {
                    month_selected_set.update(|input: &mut u32| {
                        *input -= 1;
                        if *input <= 0 {
                            *input = 12;

                            let year = year_selected.get();
                            year_selected_set.set_untracked(year - 1);
                        }
                    });

                    spawn_local(async move {
                        let lst = get_category_list(year_selected.get_untracked(), month_selected.get_untracked()).await;
                        categories.1.set(lst);
                    });
                }
            >
                <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" fill="currentColor" class="bi bi-caret-left-fill" viewBox="0 0 16 16">
                  <path d="m3.86 8.753 5.482 4.796c.646.566 1.658.106 1.658-.753V3.204a1 1 0 0 0-1.659-.753l-5.48 4.796a1 1 0 0 0 0 1.506z"/>
                </svg>
            </button>

        {move || {
                let date = chrono::Utc.with_ymd_and_hms(year_selected.get(), month_selected.get(), 1, 0, 0, 0).unwrap();
                let month_disp = date.format("%B").to_string();
                let year_disp = date.format("%G").to_string();

                let disp = format!(" {} {} ", month_disp, year_disp);
                view! {
                    <h1 class="px-3 py-4">{disp}</h1>
                }
            }
        }


            <button type="button" class="button_icon_hidden"
                on:click = move |ev| {
                    month_selected_set.update(|input: &mut u32| {
                        *input += 1;
                        if *input >= 13 {
                            *input = 1;

                            let year = year_selected.get();
                            year_selected_set.set_untracked(year + 1);
                        }
                    });

                    spawn_local(async move {
                        let lst = get_category_list(year_selected.get_untracked(), month_selected.get_untracked()).await;
                        categories.1.set(lst);
                    });
                }
            >
                <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" fill="currentColor" class="bi bi-caret-right-fill" viewBox="0 0 16 16">
                  <path d="m12.14 8.753-5.482 4.796c-.646.566-1.658.106-1.658-.753V3.204a1 1 0 0 1 1.659-.753l5.48 4.796a1 1 0 0 1 0 1.506z"/>
                </svg>
            </button>
        </div>



            <div class="row">
            <div class="col-8">

                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th scope="col">Category</th>
                            <th scope="col">Spending</th>
                        </tr>
                    </thead>
                    <tbody>
                    {
                        move || {
                            categories.0.get().into_iter().map(
                            |val| {
                                view!{
                                    <tr on:click = move |ev| {
                                        category_id_selected_set.set(val.category_id);
                                    }>
                                        <td scope="row"
                                        class:highlight = move || category_id_selected.get() == val.category_id
                                        >
                                            {val.display_name}
                                        </td>

                                        <td
                                        class:highlight = move || category_id_selected.get() == val.category_id
                                        >
                                            {data::amount_to_display(val.transaction_total * -1)}
                                        </td>
                                    </tr>
                                }

                            }
                            ).collect_view()
                        }
                    }
                    </tbody>
                </table>

                <form class="row row-cols-lg-auto" on:submit=create_category>
                    <div class="col-12">
                    <input
                        class="form-control"
                        placeholder="Enter a name..."
                        on:input=update_name
                    />
                    </div>

                    <div class="col-12">
                    <button class="btn btn-primary" type="submit">"Add Category"</button>
                    </div>

                </form>


                <div class="dropdown" data-bs-theme="dark">
                  <ul class="dropdown-menu" aria-labelledby="dropdownMenuButtonDark">
                    <li><a class="dropdown-item active" href="#">Action</a></li>
                    <li><a class="dropdown-item" href="#">Action</a></li>
                  </ul>
                </div>

            </div>
            <div class="col-4 bg-secondary-subtle rounded-3 p-3 px-4">
                <h5 class="text-secondary">"Categories Info"</h5>
                {
                    move || {
                        if category_id_selected.get() == 0 {
                            view! {
                                <h1></h1>
                                <p class="text-secondary">"Select category to view detailed info."</p>
                            }
                        } else {
                            let cats: Vec<CategoryDisplay> = categories.0.get();

                            // Get index from id
                            let cat_info: &CategoryDisplay = cats.get((category_id_selected.get() - 1) as usize).unwrap();

                            view! {
                                <h2>{&cat_info.display_name}</h2>
                                <p>"Spending Total " {data::amount_to_display(cat_info.transaction_total * -1)}</p>
                                <p>"Average (per transaction) " {data::amount_to_display((cat_info.transaction_average * -1.0) as i64)}</p>

                                <div class="card card-body d-grid gap-2">
                                    <button class="btn btn-outline-primary btn-sm"
                                        on:click = move |_| {
                                            log!("clear");
                                        }
                                    >
                                        "Rename Category"
                                    </button>
                                    <button class="btn btn-outline-danger btn-sm"
                                        on:click = move |_| {
                                            log!("clear");
                                        }
                                    >
                                        "Delete Category"
                                    </button>
                                </div>
                            }
                        }
                    }
                }
            </div>
            </div>

    }
}
