use crate::components::*;
use crate::i18n::{t, use_i18n, Locale};
use leptos::prelude::*;

#[component]
pub fn Menu() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <main>
            <nav>
                <a
                    class="menu-item"
                    href="https://github.com/reifenrath-dev/rebalax"
                    target="_blank"
                    rel="external"
                >
                    <GithubIcon />
                    {t!(i18n, source_code)}
                </a>
                <a
                    class="menu-item"
                    href="https://www.buymeacoffee.com/renereifenrath"
                    target="_blank"
                    rel="external"
                >
                    <DonateIcon />
                    {t!(i18n, donate)}
                </a>
                <a
                    class="menu-item"
                    href="https://link.reifenrath.dev/rebalax/privacy"
                    target="_blank"
                    rel="external"
                >
                    <PrivacyIcon />
                    {t!(i18n, privacy)}
                </a>
                <SwitchLang />
            </nav>
        </main>
    }
}

#[component]
pub fn SwitchLang() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="language-options">
            <b>{t!(i18n, language)}:</b>
            <input
                type="radio"
                name="language"
                id=format!("language-{}", Locale::en.to_string())
                value=Locale::en.to_string()
                checked=move || i18n.get_locale() == Locale::en
                on:change=move |_| i18n.set_locale(Locale::en)
                alt="English"
            />
            <label for=format!("language-{}", Locale::en.to_string())>English</label>
            <input
                type="radio"
                name="language"
                id=format!("language-{}", Locale::de.to_string())
                value=Locale::de.to_string()
                checked=move || i18n.get_locale() == Locale::de
                on:change=move |_| i18n.set_locale(Locale::de)
                alt="Deutsch"
            />
            <label for=format!("language-{}", Locale::de.to_string())>Deutsch</label>
        </div>
    }
}
