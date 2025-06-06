use crate::components::*;
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
                    <MenuIcon/>
                </a>
            </div>
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=Rebalancer />
                <Route path=path!("/menu") view=Menu />
            </Routes>
        </Router>
    }
}
