use crate::components::*;
use crate::functions;
use crate::i18n::*;
use crate::types::{PositionInputState, PositionsDataStore, StrategyState};
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[component]
pub fn Rebalancer() -> impl IntoView {
    let i18n = use_i18n();
    let (strategy, set_strategy, _) =
        use_local_storage::<StrategyState, JsonSerdeCodec>("strategy-state");

    let (positions, set_positions, _) =
        use_local_storage::<PositionsDataStore, JsonSerdeCodec>("asset-state");

    // Value Functions
    let position_total = move || positions.get().total();

    let target_positions = move || functions::get_target_assets(strategy.get(), positions.get());

    let strategy_options = {
        view! {
            <select
                class="select select-ghost w-min rounded-full"
                on:change:target=move |ev| set_strategy.set(ev.target().value().parse().unwrap())
                prop:value=move || strategy.get().to_string()
            >
                <option disabled>{t_string!(i18n, strategy)}</option>
                {StrategyState::iter()
                    .map(|stra| {
                        move || {
                            let active = strategy.get() == stra;
                            view! { <StrategyOption strategy=stra active /> }
                        }
                    })
                    .collect_view()}
            </select>
        }
    };

    let strategy_options_new = {
        view! {
            <div class="fab">
                <div tabindex="0" role="button" class="btn btn-lg rounded-full flex">
                    {move || format!("{}: {}", t_string!(i18n, strategy), strategy.get())}
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
                each=move || positions.get().rows.clone()
                key=|row| row.id
                children=move |position| {
                    view! {
                        <PositionRow>
                            <input
                                class="input join-item flex-grow rounded-tl-lg"
                                type="text"
                                value=position.name
                                on:input=move |ev| {
                                    let mut new_positions = positions.get().rows;
                                    new_positions
                                        .iter_mut()
                                        .find(|x| x.id == position.id)
                                        .unwrap()
                                        .name = event_target_value(&ev).parse().unwrap();
                                    set_positions
                                        .set(PositionsDataStore {
                                            rows: new_positions,
                                        })
                                }
                            />
                            <button
                                class="btn join-item rounded-tr-lg"
                                on:click=move |_| {
                                    set_positions
                                        .update(|value| {
                                            let ix = value
                                                .rows
                                                .iter()
                                                .position(|x| x.id == position.id)
                                                .unwrap();
                                            value.rows.remove(ix);
                                        })
                                }
                            >
                                <DeleteIcon />
                            </button>
                        </PositionRow>
                        <PositionRow>
                            <RowLabel>{t_string!(i18n, current)}</RowLabel>
                            <InputCell>
                                <input
                                    class="input text-right"
                                    id=format!("{}-position-input", position.id)
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
                                        let mut new_positions = positions.get().rows;
                                        new_positions
                                            .iter_mut()
                                            .find(|x| x.id == position.id)
                                            .unwrap()
                                            .current_position = event_target_value(&ev)
                                            .parse::<Decimal>()
                                            .unwrap_or(dec!(0));
                                        set_positions
                                            .set(PositionsDataStore {
                                                rows: new_positions,
                                            })
                                    }
                                />
                            </InputCell>
                            <DisplayCell>
                                <div class="text-right">
                                    {move || {
                                        format!(
                                            "{} %",
                                            (positions.get().allocation_for(position.id) * dec!(100))
                                                .round_dp(2),
                                        )
                                    }}
                                </div>
                            </DisplayCell>
                        </PositionRow>
                        <PositionRow>
                            <RowLabel>{t_string!(i18n, target)}</RowLabel>
                            <DisplayCell>
                                <div class="text-right">
                                    {move || {
                                        target_positions()
                                            .iter()
                                            .find(|x| x.id == position.id)
                                            .unwrap()
                                            .value
                                            .round_dp(0)
                                            .to_string()
                                    }}
                                    {move || {
                                        let diff = (target_positions()
                                            .iter()
                                            .find(|x| x.id == position.id)
                                            .unwrap()
                                            .value
                                            - positions
                                                .get()
                                                .rows
                                                .iter()
                                                .find(|x| x.id == position.id)
                                                .unwrap()
                                                .current_position)
                                            .round_dp(0);
                                        view! { <DiffString diff has_braces=true /> }
                                    }}
                                </div>
                            </DisplayCell>
                            <InputCell>
                                <label class="input text-right">
                                    <input
                                        id=format!("{}-target-input", position.id)
                                        min="0.01"
                                        max="100"
                                        placeholder="..."
                                        type="number"
                                        class="grow text-right"
                                        value=if position.target_allocation.is_zero() {
                                            "".to_string()
                                        } else {
                                            (position.target_allocation * dec!(100))
                                                .round_dp(2)
                                                .to_string()
                                        }
                                        on:input=move |ev| {
                                            let mut new_positions = positions.get().rows;
                                            new_positions
                                                .iter_mut()
                                                .find(|x| x.id == position.id)
                                                .unwrap()
                                                .target_allocation = event_target_value(&ev)
                                                .parse::<Decimal>()
                                                .unwrap_or(dec!(0)) / dec!(100);
                                            set_positions
                                                .set(PositionsDataStore {
                                                    rows: new_positions,
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
                class="btn w-full rounded-b-lg"
                on:click=move |_| {
                    let len = positions.get().rows.len();
                    set_positions
                        .update(|value| {
                            *value = PositionsDataStore {
                                rows: value
                                    .rows
                                    .iter()
                                    .cloned()
                                    .chain([
                                        PositionInputState {
                                            id: Uuid::now_v7(),
                                            name: format!("Position {}", len + 1),
                                            current_position: dec!(0),
                                            target_allocation: dec!(0),
                                        },
                                    ])
                                    .collect(),
                            };
                        })
                }
            >
                <PlusIcon />
            </button>
        }
    };

    let total_calculation_string = move || {
        let diff = (position_total()
            - (target_positions()
                .iter()
                .cloned()
                .fold(dec!(0), |acc, x| acc + x.value.round_dp(0))))
            * dec!(-1);
        if strategy.get() == StrategyState::Reallocate
            || !positions.get().is_valid_target_allocation()
            || !positions.get().all_positions_above_zero()
            || diff == dec!(0)
        {
            view! { {position_total().to_string()} }.into_any()
        } else {
            view! {
                {position_total().to_string()}
                <DiffString diff has_braces=false />
                {" = ".to_string()}
                {target_positions()
                    .iter()
                    .cloned()
                    .fold(dec!(0), |acc, x| acc + x.value.round_dp(0))
                    .to_string()}
            }
            .into_any()
        }
    };

    let title_bar = view! {
        <ul class="menu menu-horizontal bg-base-200 rounded-full">
            <li>{strategy_options}</li>
        </ul>
        <ul class="menu menu-horizontal bg-base-200 rounded-full">
            <li>
                <SwitchMenuButton />
            </li>
        </ul>
    };

    view! {
        <TitleBar>{title_bar}</TitleBar>
        <main class="pt-[100px]">
            <section>{position_table_rows}</section>

            <section class="pt-[5px]">{add_position_button}</section>

            <section class="flex mt-5">
                <b class="grow">{t!(i18n, total)}</b>
                <span>{total_calculation_string}</span>
            </section>

            {strategy_options_new}
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
