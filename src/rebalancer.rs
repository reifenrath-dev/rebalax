use crate::components::icons::{DeleteIcon, NextIcon, PlusIcon, PreviousIcon};
use crate::components::*;
use crate::functions;
use crate::i18n::*;
use crate::types::extensions::DecimalExtensions;
use crate::types::portfolios_data_store::PortfoliosDataStore;
use crate::types::strategy_state::StrategyState;
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use rust_decimal_macros::dec;
use uuid::Uuid;

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
                <button
                    class="rounded-l-full btn btn-circle join-item"
                    on:click=move |_| {
                        let portfolio_data = portfolios.get();
                        let selected_portfolio = portfolio_data.selected_portfolio;
                        let previous_portfolio = portfolio_data.previous_or_selected_portfolio();
                        if selected_portfolio != previous_portfolio {
                            set_portfolios
                                .update(|portfolios_data_store| {
                                    portfolios_data_store.selected_portfolio = previous_portfolio;
                                })
                        }
                    }
                    disabled=move || {
                        let portfolio_data = portfolios.get();
                        match portfolio_data.portfolios.first_key_value() {
                            Some((&first_key, _)) => portfolio_data.selected_portfolio == first_key,
                            None => false,
                        }
                    }
                >
                    <PreviousIcon />
                </button>
                <input
                    type="text"
                    class="font-bold input grow join-item"
                    value=move || portfolios.get().get_current().name.to_string()
                    on:input=move |ev| {
                        set_portfolios
                            .update(|portfolios_data_store| {
                                portfolios_data_store.set_portfolio_name(event_target_value(&ev))
                            });
                    }
                    placeholder=move || t_string!(i18n, portfolio_placeholder)
                />
                <button
                    class="btn btn-circle join-item"
                    on:click=move |_| {
                        let portfolio_data = portfolios.get();
                        let selected_portfolio = portfolio_data.selected_portfolio;
                        let fallback_portfolio = portfolio_data.fallback_portfolio();
                        set_portfolios
                            .update(|portfolios_data_store| {
                                portfolios_data_store.remove_portfolio(selected_portfolio);
                                portfolios_data_store.selected_portfolio = fallback_portfolio;
                            })
                    }
                    disabled=move || { portfolios.get().portfolios.len() == 1 }
                >
                    <DeleteIcon />
                </button>
                <button
                    class="btn btn-circle join-item"
                    on:click=move |_| {
                        set_portfolios
                            .update(|portfolios_data_store| {
                                let id = Uuid::now_v7();
                                portfolios_data_store.add_portfolio(id, Default::default());
                                portfolios_data_store.selected_portfolio = id;
                            })
                    }
                >
                    <PlusIcon />
                </button>
                <button
                    class="rounded-r-full btn btn-circle join-item"
                    on:click=move |_| {
                        let portfolio_data = portfolios.get();
                        let selected_portfolio = portfolio_data.selected_portfolio;
                        let next_portfolio = portfolio_data.next_or_selected_portfolio();
                        if next_portfolio != selected_portfolio {
                            set_portfolios
                                .update(|portfolios_data_store| {
                                    portfolios_data_store.selected_portfolio = next_portfolio;
                                })
                        }
                    }
                    disabled=move || {
                        let portfolio_data = portfolios.get();
                        match portfolio_data.portfolios.last_key_value() {
                            Some((&last_key, _)) => portfolio_data.selected_portfolio == last_key,
                            None => false,
                        }
                    }
                >
                    <NextIcon />
                </button>
            </div>
            <SwitchMenuButton />
        </div>
        <main class="pt-[100px]">
            <section>
                <ListOfPositions portfolios set_portfolios strategy />
            </section>

            <section class="pt-[5px]">
                <AddPositionButton set_portfolios />
            </section>

            <section class="flex mt-5">
                <b class="grow">{t!(i18n, total)}</b>
                <span>{total_calculation_string}</span>
            </section>

            <StrategySelector strategy set_strategy />
        </main>
    }
}

#[component]
pub fn ListOfPositions(
    portfolios: Signal<PortfoliosDataStore>,
    set_portfolios: WriteSignal<PortfoliosDataStore>,
    strategy: Signal<StrategyState>,
) -> impl IntoView {
    let i18n = use_i18n();

    let target_portfolio =
        move || functions::get_target_assets(strategy.get(), portfolios.get().get_current());
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
                            placeholder=move || t_string!(i18n, asset_placeholder)
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
                            <CurrentPositionInput
                                position_id
                                current_position=position.current_position
                                set_portfolios
                            />
                        </InputCell>
                        <DisplayCell>
                            <div class="text-right">
                                {move || {
                                    format!(
                                        "{} %",
                                        portfolios
                                            .get()
                                            .get_current()
                                            .allocation_for(&position_id)
                                            .to_percentage(),
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
                            <TargetAllocationInput
                                position_id
                                target_allocation=position.target_allocation
                                set_portfolios
                            />
                        </InputCell>
                    </PositionRow>
                }
            }
        />
    }
}
