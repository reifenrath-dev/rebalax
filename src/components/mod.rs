pub mod icons;

use crate::components::icons::*;
use crate::i18n::*;
use crate::types::extensions::{DecimalExtensions, EventExtensions};
use crate::types::portfolios_data_store::PortfoliosDataStore;
use crate::types::strategy_state::StrategyState;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use uuid::Uuid;

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
        view! { <span class="text-success">{fmt}</span> }
    } else {
        let fmt = if has_braces {
            format!(" ({})", diff)
        } else {
            format!(" {}", diff)
        };
        view! { <span class="text-error">{fmt}</span> }
    }
}

#[component]
pub fn StrategyButton(
    strategy: StrategyState,
    active: bool,
    on_click: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let i18n = use_i18n();
    let value = match strategy {
        StrategyState::Buy => t!(i18n, alt_buy).into_any(),
        StrategyState::Reallocate => t!(i18n, alt_buy_sell).into_any(),
        StrategyState::Sell => t!(i18n, alt_sell).into_any(),
    };

    if active {
        view! {
            <button disabled class="rounded-full btn bg-base-100">
                {value}
                <StrategyIcon strategy=strategy />
            </button>
        }
        .into_any()
    } else {
        view! {
            <button class="rounded-full btn" on:click=on_click>
                {value}
                <StrategyIcon strategy=strategy />
            </button>
        }
        .into_any()
    }
}

#[component]
pub fn TitleBar(children: Children) -> impl IntoView {
    view! { <div class="fixed w-full ml-[-20px] p-[20px] z-100">{children()}</div> }
}

#[component]
pub fn SwitchMenuButton() -> impl IntoView {
    move || {
        if use_location().pathname.get().contains("menu") {
            view! {
                <a href="/" class="rounded-full btn">
                    <CloseIcon />
                </a>
            }
            .into_any()
        } else {
            view! {
                <a href="/menu" class="rounded-full btn">
                    <MenuIcon />
                </a>
            }
            .into_any()
        }
    }
}

#[component]
pub fn StrategySelector(
    strategy: Signal<StrategyState>,
    set_strategy: WriteSignal<StrategyState>,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="fab">
            <div tabindex="0" role="button" class="flex rounded-full btn btn-lg">
                {t!(i18n, strategy)}
                :
                {move || view! { <StrategyIcon strategy=strategy.get() /> }}
            </div>
            {StrategyState::iter()
                .map(|stra| {
                    move || {
                        let active = strategy.get() == stra;
                        view! {
                            <StrategyButton
                                strategy=stra
                                active
                                on_click=move |_| set_strategy.set(stra)
                            />
                        }
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn TargetAllocationInput(
    position_id: Uuid,
    target_allocation: Decimal,
    set_portfolios: WriteSignal<PortfoliosDataStore>,
) -> impl IntoView {
    view! {
        <label class="text-right input">
            <input
                id=format!("{}-target-input", position_id)
                min="0.01"
                max="100"
                placeholder="..."
                type="number"
                class="text-right grow"
                value=if target_allocation.is_zero() {
                    "".to_string()
                } else {
                    target_allocation.to_percentage().to_string()
                }
                on:input=move |ev| {
                    set_portfolios
                        .update(|portfolios_data_store| {
                            portfolios_data_store
                                .set_position_target_allocation(
                                    &position_id,
                                    event_target_value(&ev).parse().unwrap_or(dec!(0)) / dec!(100),
                                )
                        })
                }
            />
            <span class="ml-[-4px]">%</span>
        </label>
    }
}

#[component]
pub fn CurrentPositionInput(
    position_id: Uuid,
    current_position: Decimal,
    set_portfolios: WriteSignal<PortfoliosDataStore>,
) -> impl IntoView {
    view! {
        <input
            class="text-right input"
            id=format!("{}-position-input", position_id)
            min="0"
            max="9999999"
            placeholder="..."
            type="number"
            value=if current_position.is_zero() {
                "".to_string()
            } else {
                current_position.round_dp(0).to_string()
            }
            on:input=move |ev| {
                set_portfolios
                    .update(|portfolios_data_store| {
                        portfolios_data_store
                            .set_position_current_value(&position_id, ev.to_decimal_or_zero())
                    })
            }
        />
    }
}

#[component]
pub fn AddPositionButton(set_portfolios: WriteSignal<PortfoliosDataStore>) -> impl IntoView {
    view! {
        <button
            class="w-full rounded-b-lg btn"
            on:click=move |_| {
                set_portfolios
                    .update(|portfolios_data_store| {
                        portfolios_data_store.add_position(Default::default())
                    })
            }
        >
            <PlusIcon />
        </button>
    }
}

#[component]
pub fn PositionRow(children: Children) -> impl IntoView {
    view! { <div class="flex items-center h-[3rem]">{children()}</div> }
}

#[component]
pub fn RowLabel(children: Children) -> impl IntoView {
    view! { <div class="grow px-[12px]">{children()}</div> }
}

#[component]
pub fn DisplayCell(children: Children) -> impl IntoView {
    view! { <div class="flex-none w-[150px] px-[12px]">{children()}</div> }
}

#[component]
pub fn InputCell(children: Children) -> impl IntoView {
    view! { <div class="flex-none w-[150px]">{children()}</div> }
}
