use crate::components::*;
use crate::i18n::{t, use_i18n, Locale};
use leptos::prelude::*;

#[component]
pub fn Menu() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <main>
            <TitleBar>
                <div class="float-right">
                    <SwitchMenuButton />
                </div>
            </TitleBar>
            <div class="pt-[100px] flex flex-col gap-4">
                <MenuItem href="https://github.com/reifenrath-dev/rebalax">
                    <GithubIcon />
                    {t!(i18n, source_code)}
                </MenuItem>
                <MenuItem href="https://www.buymeacoffee.com/renereifenrath">
                    <DonateIcon />
                    {t!(i18n, donate)}
                </MenuItem>
                <MenuItem href="https://link.reifenrath.dev/rebalax/privacy">
                    <PrivacyIcon />
                    {t!(i18n, privacy)}
                </MenuItem>
                <SwitchLang />
            </div>
        </main>
    }
}

#[component]
pub fn MenuItem(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <a class="btn" href=href target="_blank" rel="external">
            {children()}
        </a>
    }
}

#[component]
pub fn SwitchLang() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="p-[20px]">
            <b>{t!(i18n, language)}:</b>
            <ul class="list">
                <li class="list-row flex">
                    <label class="grow" for=format!("language-{}", Locale::en.to_string())>
                        English
                    </label>
                    <input
                        type="radio"
                        class="radio"
                        name="language"
                        id=format!("language-{}", Locale::en.to_string())
                        value=Locale::en.to_string()
                        checked=move || i18n.get_locale() == Locale::en
                        on:change=move |_| i18n.set_locale(Locale::en)
                        alt="English"
                    />
                </li>
                <li class="list-row flex">
                    <label class="grow" for=format!("language-{}", Locale::de.to_string())>
                        Deutsch
                    </label>
                    <input
                        type="radio"
                        class="radio"
                        name="language"
                        id=format!("language-{}", Locale::de.to_string())
                        value=Locale::de.to_string()
                        checked=move || i18n.get_locale() == Locale::de
                        on:change=move |_| i18n.set_locale(Locale::de)
                        alt="Deutsch"
                    />
                </li>
            </ul>
        </div>
    }
}
