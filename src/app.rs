use crate::components::*;
use crate::i18n::*;
use crate::menu::Menu;
use crate::rebalancer::Rebalancer;
use leptos::prelude::*;
use leptos_i18n_router::I18nRoute;
use leptos_router::{components::*, hooks::use_location, path};

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <I18nContextProvider>
            <Router>
                <TitleBar />
                <Routes fallback=|| "Not found">
                    <I18nRoute<Locale, _, _> view=|| view! { <Outlet /> }>
                        <Route path=path!("/") view=Rebalancer />
                        <Route path=path!("/menu") view=Menu />
                    </I18nRoute<Locale, _, _>>
                </Routes>
            </Router>
        </I18nContextProvider>
    }
}

#[component]
pub fn TitleBar() -> impl IntoView {
    view! {
        <div id="titlebar-container">
            <div id="titlebar-spacer"></div>
            <div class="titlebar">
                <a class="titlebar-button" href="/">
                    <img height=24 width=24 src="public/32x32.png" draggable="false" />
                </a>
                <a class="titlebar-button" href="/">
                    <span class="titlebar-title">Rebalax</span>
                </a>
                <SwitchMenuButton />
            </div>
        </div>
    }
}

#[component]
pub fn SwitchMenuButton() -> impl IntoView {
    move || {
        if use_location().pathname.get().contains("menu") {
            view! {
                <a class="titlebar-button" id="titlebar-menu" href="/">
                    <CloseIcon />
                </a>
            }
            .into_any()
        } else {
            view! {
                <a class="titlebar-button" id="titlebar-menu" href="/menu">
                    <MenuIcon />
                </a>
            }
            .into_any()
        }
    }
}
