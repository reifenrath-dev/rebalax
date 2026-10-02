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
        <a class="btn justify-start" href=href target="_blank" rel="external">
            {children()}
        </a>
    }
}

#[component]
pub fn SwitchLang() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="p-[20px] bg-base-100">
            <div class="flex gap-1">
                <LanguageIcon />
                <b>{t!(i18n, language)}:</b>
            </div>
            <ul>
                <LanguageOption locale=Locale::en alt=String::from("English") />
                <LanguageOption locale=Locale::de alt=String::from("Deutsch") />
            </ul>
        </div>
    }
}

#[component]
pub fn LanguageOption(locale: Locale, alt: String) -> impl IntoView {
    let locale_string = locale.to_string();

    view! {
        <li class="flex pt-2">
            <label class="grow" for=format!("language-{}", locale_string)>
                {alt.clone()}
            </label>
            <input
                type="radio"
                class="radio border-2"
                name="language"
                id=format!("language-{}", locale_string)
                value=locale_string
                checked=move || use_i18n().get_locale() == locale
                on:change=move |_| use_i18n().set_locale(locale)
                alt=alt
            />
        </li>
    }
}
