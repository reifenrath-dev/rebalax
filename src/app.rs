use crate::menu::Menu;
use crate::rebalancer::Rebalancer;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="titlebar">
                <img height=24 width=24 src="public/32x32.png" />
                <span class="titlebar-title">Rebalax</span>
                <a
                    class="titlebar-button"
                    id="titlebar-menu"
                    href=move || {
                        if use_location().pathname.get().contains("menu") { "/" } else { "/menu" }
                    }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="lucide lucide-menu-icon lucide-menu"
                    >
                        <path d="M4 12h16" />
                        <path d="M4 18h16" />
                        <path d="M4 6h16" />
                    </svg>
                </a>
            </div>
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=Rebalancer />
                <Route path=path!("/menu") view=Menu />
            </Routes>
        </Router>
    }
}
