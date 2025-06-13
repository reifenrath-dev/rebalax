use crate::components::*;
use crate::i18n::{t, use_i18n, Locale};
use leptos::prelude::*;

#[component]
pub fn Menu() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <main>
            <nav>
                <a class="menu-item" href="https://github.com/reifenrath-dev/rebalax" target="_blank" rel="external">
                    <GithubIcon/>{t!(i18n, source_code)}
                </a>
                <a class="menu-item" href="https://www.buymeacoffee.com/renereifenrath" target="_blank" rel="external">
                    <DonateIcon/>{t!(i18n, donate)}
                </a>
                <SwitchLang />
            </nav>
        </main>
    }
}

#[component]
pub fn SwitchLang() -> impl IntoView {
    let i18n = use_i18n();

    let on_switch = move |_| {
        let new_lang = match i18n.get_locale() {
            Locale::en => Locale::de,
            Locale::de => Locale::en,
        };
        i18n.set_locale(new_lang);
    };

    view! {
        <button on:click=on_switch>{t!(i18n, click_to_change_lang)}</button>
    }
}
