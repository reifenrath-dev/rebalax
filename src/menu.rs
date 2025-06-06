use crate::components::*;
use leptos::prelude::*;

#[component]
pub fn Menu() -> impl IntoView {
    view! {
        <main>
            <nav>
                <a class="menu-item" href="https://github.com/reifenrath-dev/rebalax" target="_blank" rel="external">
                    <GithubIcon/>Github
                </a>
                <a class="menu-item" href="https://www.buymeacoffee.com/renereifenrath" target="_blank" rel="external">
                    <DonateIcon/>Donate
                </a>
            </nav>
        </main>
    }
}
