use crate::i18n::*;
use crate::menu::Menu;
use crate::rebalancer::Rebalancer;
use leptos::prelude::*;
use leptos_i18n_router::I18nRoute;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <I18nContextProvider>
            <Router>
                <div class="bg-black w-full p-[20px]">
                    <Routes fallback=|| "Not found">
                        <I18nRoute<Locale, _, _> view=|| view! { <Outlet /> }>
                            <Route path=path!("/") view=Rebalancer />
                            <Route path=path!("/menu") view=Menu />
                        </I18nRoute<Locale, _, _>>
                    </Routes>
                </div>
            </Router>
        </I18nContextProvider>
    }
}
