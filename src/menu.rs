use crate::components::*;
use crate::i18n::{t, use_i18n, Locale};
use leptos::prelude::*;

#[component]
pub fn Menu() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <main>
            <div class="fixed w-full ml-[-20px] p-[20px] z-100">
                <ul class="menu menu-horizontal bg-base-200 rounded-full float-right">
                    <li>
                        <SwitchMenuButton />
                    </li>
                </ul>
            </div>
            <div class="pt-[100px]">
                <ul class="menu w-full bg-base-100 rounded-lg">
                    <li>
                        <a
                            class="menu-item"
                            href="https://github.com/reifenrath-dev/rebalax"
                            target="_blank"
                            rel="external"
                        >
                            <GithubIcon />
                            {t!(i18n, source_code)}
                        </a>
                    </li>
                    <li>
                        <a
                            class="menu-item"
                            href="https://www.buymeacoffee.com/renereifenrath"
                            target="_blank"
                            rel="external"
                        >
                            <DonateIcon />
                            {t!(i18n, donate)}
                        </a>
                    </li>
                    <li>
                        <a
                            class="menu-item"
                            href="https://link.reifenrath.dev/rebalax/privacy"
                            target="_blank"
                            rel="external"
                        >
                            <PrivacyIcon />
                            {t!(i18n, privacy)}
                        </a>
                    </li>
                </ul>
                <SwitchLang />
            </div>
        </main>
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
