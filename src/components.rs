use crate::i18n::*;
use crate::types::StrategyState;
use leptos::prelude::*;
use rust_decimal::Decimal;

#[component]
pub fn MenuIcon() -> impl IntoView {
    view! {
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
    }
}

#[component]
pub fn GithubIcon() -> impl IntoView {
    view! {
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
            class="lucide lucide-github-icon lucide-github"
        >
            <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" />
            <path d="M9 18c-4.51 2-5-2-7-2" />
        </svg>
    }
}

#[component]
pub fn DonateIcon() -> impl IntoView {
    view! {
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
            class="lucide lucide-gift-icon lucide-gift"
        >
            <rect x="3" y="8" width="18" height="4" rx="1" />
            <path d="M12 8v13" />
            <path d="M19 12v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-7" />
            <path d="M7.5 8a2.5 2.5 0 0 1 0-5A4.8 8 0 0 1 12 8a4.8 8 0 0 1 4.5-5 2.5 2.5 0 0 1 0 5" />
        </svg>
    }
}

#[component]
pub fn DeleteIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="1rem"
            height="1rem"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-trash-icon lucide-trash"
        >
            <path d="M3 6h18" />
            <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
            <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
        </svg>
    }
}

#[component]
pub fn PlusIcon() -> impl IntoView {
    view! {
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
            class="lucide lucide-plus-icon lucide-plus"
        >
            <path d="M5 12h14" />
            <path d="M12 5v14" />
        </svg>
    }
}

#[component]
pub fn MinusIcon() -> impl IntoView {
    view! {
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
            class="lucide lucide-minus-icon lucide-minus"
        >
            <path d="M5 12h14" />
        </svg>
    }
}

#[component]
pub fn PlusMinusIcon() -> impl IntoView {
    view! {
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
            class="lucide lucide-diff-icon lucide-diff"
        >
            <path d="M12 3v14" />
            <path d="M5 10h14" />
            <path d="M5 21h14" />
        </svg>
    }
}

#[component]
pub fn CloseIcon() -> impl IntoView {
    view! {
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
            class="lucide lucide-x-icon lucide-x"
        >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
        </svg>
    }
}

#[component]
pub fn PrivacyIcon() -> impl IntoView {
    view! {
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
            class="lucide lucide-hat-glasses-icon lucide-hat-glasses"
        >
            <path d="M14 18a2 2 0 0 0-4 0" />
            <path d="m19 11-2.11-6.657a2 2 0 0 0-2.752-1.148l-1.276.61A2 2 0 0 1 12 4H8.5a2 2 0 0 0-1.925 1.456L5 11" />
            <path d="M2 11h20" />
            <circle cx="17" cy="18" r="3" />
            <circle cx="7" cy="18" r="3" />
        </svg>
    }
}

#[component]
pub fn DiffString(diff: Decimal, has_braces: bool) -> impl IntoView {
    if diff.is_zero() {
        view! { <span class="zero">{"".to_string()}</span> }
    } else if diff.is_sign_positive() {
        let fmt = if has_braces {
            format!(" (+{})", diff)
        } else {
            format!(" +{}", diff)
        };
        view! { <span class="positive">{fmt}</span> }
    } else {
        let fmt = if has_braces {
            format!(" ({})", diff)
        } else {
            format!(" {}", diff)
        };
        view! { <span class="negative">{fmt}</span> }
    }
}

#[component]
pub fn StrategyOption(strategy: StrategyState, active: bool) -> impl IntoView {
    let i18n = use_i18n();

    if active {
        view! {
            <span class="active">
                {match strategy {
                    StrategyState::Buy => t_string!(i18n, alt_buy),
                    StrategyState::BuySell => t_string!(i18n, alt_buy_sell),
                    StrategyState::Sell => t_string!(i18n, alt_sell),
                }}
            </span>
        }
        .into_any()
    } else {
        view! {
            <span class="in-active">
                {match strategy {
                    StrategyState::Buy => view! { <PlusIcon /> }.into_any(),
                    StrategyState::BuySell => view! { <PlusMinusIcon /> }.into_any(),
                    StrategyState::Sell => view! { <MinusIcon /> }.into_any(),
                }}
            </span>
        }
        .into_any()
    }
}
