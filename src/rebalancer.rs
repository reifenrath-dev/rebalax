use crate::components::*;
use crate::functions;
use crate::i18n::*;
use crate::types::{PortfoliosDataStore, Position, StrategyState};
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;

#[component]
pub fn Rebalancer() -> impl IntoView {
    let i18n = use_i18n();
    let (strategy, set_strategy, _) =
        use_local_storage::<StrategyState, JsonSerdeCodec>("strategy-state");

    let (portfolios, set_portfolios, _) =
        use_local_storage::<PortfoliosDataStore, JsonSerdeCodec>("asset-state");

    // Value Functions
    let portfolio_total = move || portfolios.get().get_current().total();

    let target_portfolio =
        move || functions::get_target_assets(strategy.get(), portfolios.get().get_current());

    let strategy_options = {
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
    };

    let position_table_rows = {
        view! {
            <For
                each=move || portfolios.get().get_current().positions.clone()
                key=|(row_id, _)| row_id.to_string()
                children=move |(position_id, position)| {
                    view! {
                        <PositionRow>
                            <input
                                class="flex-grow rounded-tl-lg input join-item"
                                type="text"
                                value=position.name.to_string()
                                on:input=move |ev| {
                                    set_portfolios
                                        .update(|portfolios_data_store| {
                                            portfolios_data_store
                                                .set_position_name(&position_id, event_target_value(&ev))
                                        })
                                }
                            />
                            <button
                                class="rounded-tr-lg btn join-item"
                                on:click=move |_| {
                                    set_portfolios
                                        .update(|portfolios_data_store| {
                                            portfolios_data_store.remove_position(&position_id)
                                        })
                                }
                            >
                                <DeleteIcon />
                            </button>
                        </PositionRow>
                        <PositionRow>
                            <RowLabel>{t!(i18n, current)}</RowLabel>
                            <InputCell>
                                <input
                                    class="text-right input"
                                    id=format!("{}-position-input", position_id)
                                    min="0"
                                    max="9999999"
                                    placeholder="..."
                                    type="number"
                                    value=if position.current_position.is_zero() {
                                        "".to_string()
                                    } else {
                                        position.current_position.round_dp(0).to_string()
                                    }
                                    on:input=move |ev| {
                                        set_portfolios
                                            .update(|portfolios_data_store| {
                                                portfolios_data_store
                                                    .set_position_current_value(
                                                        &position_id,
                                                        event_target_value(&ev).parse().unwrap_or(dec!(0)),
                                                    )
                                            })
                                    }
                                />
                            </InputCell>
                            <DisplayCell>
                                <div class="text-right">
                                    {move || {
                                        format!(
                                            "{} %",
                                            (portfolios.get().get_current().allocation_for(&position_id)
                                                * dec!(100))
                                                .round_dp(2),
                                        )
                                    }}
                                </div>
                            </DisplayCell>
                        </PositionRow>
                        <PositionRow>
                            <RowLabel>{t!(i18n, target)}</RowLabel>
                            <DisplayCell>
                                <div class="text-right">
                                    {move || {
                                        target_portfolio()[&position_id].round_dp(0).to_string()
                                    }}
                                    {move || {
                                        let diff = (target_portfolio()[&position_id]
                                            - portfolios
                                                .get()
                                                .get_current()
                                                .positions[&position_id]
                                                .current_position)
                                            .round_dp(0);
                                        view! { <DiffString diff has_braces=true /> }
                                    }}
                                </div>
                            </DisplayCell>
                            <InputCell>
                                <label class="text-right input">
                                    <input
                                        id=format!("{}-target-input", position_id)
                                        min="0.01"
                                        max="100"
                                        placeholder="..."
                                        type="number"
                                        class="text-right grow"
                                        value=if position.target_allocation.is_zero() {
                                            "".to_string()
                                        } else {
                                            (position.target_allocation * dec!(100))
                                                .round_dp(2)
                                                .to_string()
                                        }
                                        on:input=move |ev| {
                                            set_portfolios
                                                .update(|portfolios_data_store| {
                                                    portfolios_data_store
                                                        .set_position_target_allocation(
                                                            &position_id,
                                                            event_target_value(&ev).parse().unwrap_or(dec!(0))
                                                                / dec!(100),
                                                        )
                                                })
                                        }
                                    />
                                    <span class="ml-[-4px]">%</span>
                                </label>
                            </InputCell>
                        </PositionRow>
                    }
                }
            />
        }
    };

    let add_position_button = {
        view! {
            <button
                class="w-full rounded-b-lg btn"
                on:click=move |_| {
                    set_portfolios
                        .update(|portfolios_data_store| {
                            portfolios_data_store
                                .add_position(Position {
                                    name: "".to_string(),
                                    current_position: dec!(0),
                                    target_allocation: dec!(0),
                                })
                        })
                }
            >
                <PlusIcon />
            </button>
        }
    };

    let total_calculation_string = move || {
        let diff = (portfolio_total()
            - (target_portfolio()
                .values()
                .fold(dec!(0), |acc, x| acc + x.round_dp(0))))
            * dec!(-1);
        if strategy.get() == StrategyState::Reallocate
            || !portfolios.get().get_current().is_valid_target_allocation()
            || !portfolios.get().get_current().all_positions_above_zero()
            || diff == dec!(0)
        {
            view! { {portfolio_total().to_string()} }.into_any()
        } else {
            view! {
                {portfolio_total().to_string()}
                <DiffString diff has_braces=false />
                {" = ".to_string()}
                {target_portfolio()
                    .values()
                    .fold(dec!(0), |acc, x| acc + x.round_dp(0))
                    .to_string()}
            }
            .into_any()
        }
    };

    view! {
        <div class="flex fixed gap-2 w-full ml-[-20px] p-[20px] z-100">
            <div class="join grow">
                <button class="rounded-l-full btn btn-circle join-item">
                    <PreviousIcon />
                </button>
                <input
                    type="text"
                    class="font-bold input grow join-item"
                    value=portfolios.get().get_current().name.to_string()
                    on:input=move |ev| {
                        set_portfolios
                            .update(|portfolios_data_store| {
                                portfolios_data_store.set_portfolio_name(event_target_value(&ev))
                            });
                    }
                />
                <button class="btn btn-circle join-item">
                    <PlusIcon />
                </button>
                <button class="rounded-r-full btn btn-circle join-item">
                    <NextIcon />
                </button>
            </div>
            <SwitchMenuButton />
        </div>
        <main class="pt-[100px]">
            <section>{position_table_rows}</section>

            <section class="pt-[5px]">{add_position_button}</section>

            <section class="flex mt-5">
                <b class="grow">{t!(i18n, total)}</b>
                <span>{total_calculation_string}</span>
            </section>

            {strategy_options}
        </main>
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
