use leptos::leptos_dom::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
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
                    <li class="nav-item">
                        <button type="button" class="btn btn-outline-secondary btn-sm">Add Account</button>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
