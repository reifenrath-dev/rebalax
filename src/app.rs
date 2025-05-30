use crate::functions;
use crate::types::{PositionInputState, PositionsDataStore, StrategyState};
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    let (strategy, set_strategy, _) =
        use_local_storage::<StrategyState, JsonSerdeCodec>("strategy-state");

    let (positions, set_positions, _) =
        use_local_storage::<PositionsDataStore, JsonSerdeCodec>("asset-state");

    // Value Functions
    let position_total = move || positions.get().total();

    let target_positions = move || functions::get_target_assets(strategy.get(), positions.get());

    let get_diff_string = |diff: Decimal| {
        if diff.is_zero() {
            view! { <span class="zero">{"".to_string()}</span> }
        } else {
            if diff.is_sign_positive() {
                view! { <span class="positive">{format!(" +{}", diff)}</span> }
            } else {
                view! { <span class="negative">{format!(" {}", diff)}</span> }
            }
        }
    };

    let get_diff_string_with_braces = |diff: Decimal| {
        if diff.is_zero() {
            view! { <span class="zero">{"".to_string()}</span> }
        } else {
            if diff.is_sign_positive() {
                view! { <span class="positive">{format!(" (+{})", diff)}</span> }
            } else {
                view! { <span class="negative">{format!(" ({})", diff)}</span> }
            }
        }
    };

    view! {
        <div class="titlebar">
            <img height=24 width=24 src="public/32x32.png"/>
            <span class="titlebar-title">Rebalax</span>
            <div class="titlebar-button" id="titlebar-menu">
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
            </div>
        </div>

        <main>
            <section class="strategy">
                <b>Strategy</b>
                <div class="strategy-options">
                    {StrategyState::iter()
                        .map(|stra| {
                            view! {
                                <input
                                    type="radio"
                                    name="strategy"
                                    id=format!("strategy-{}", stra)
                                    value=stra.to_string()
                                    checked=move || strategy.get() == stra
                                    on:change=move |_| set_strategy.set(stra)
                                />
                                <label for=format!(
                                    "strategy-{}",
                                    stra,
                                )>
                                    {if stra == StrategyState::BuySell {
                                        "Buy & Sell".to_string()
                                    } else {
                                        stra.to_string()
                                    }}
                                </label>
                            }
                        })
                        .collect_view()}
                </div>
            </section>

            <table>
                <For
                    each=move || positions.get().rows.clone()
                    key=|row| row.id.clone()
                    children=move |position| {
                        view! {
                            <tr>
                                <td colspan=3 class="title">
                                    <div class="title-input-container">
                                        <input
                                            class="title-input"
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
                                            class="remove-position"
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
                                        </button>
                                    </div>
                                </td>
                            </tr>
                            <tr class="current">
                                <td>Current</td>
                                <td class="number">
                                    <input
                                        id=format!("{}-position-input", position.id)
                                        min="0"
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
                                </td>
                                <td class="number">
                                    <div class="number percentage">
                                        {move || {
                                            format!(
                                                "{}",
                                                (positions.get().allocation_for(position.id) * dec!(100))
                                                    .round_dp(2)
                                                    .to_string(),
                                            )
                                        }}
                                    </div>
                                </td>
                            </tr>
                            <tr class="target">
                                <td>Target</td>
                                <td class="number">
                                    <div class="number">
                                        {move || {
                                            target_positions()
                                                .iter()
                                                .find(|x| x.id == position.id)
                                                .unwrap()
                                                .value
                                                .round_dp(0)
                                                .to_string()
                                        }}
                                        {move || get_diff_string_with_braces(
                                            (target_positions()
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
                                                .round_dp(0),
                                        )}
                                    </div>
                                </td>
                                <td class="number">
                                    <input
                                        id=format!("{}-target-input", position.id)
                                        min="0"
                                        placeholder="..."
                                        type="number"
                                        class="percentage"
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
                                </td>
                            </tr>
                        }
                    }
                />
            </table>

            <section class="add-remove">
                <button
                    class="add-position"
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
                </button>
            </section>

            <section class="total">
                <b>Total</b>
                <span>
                    {move || {
                        let diff = ((position_total()
                            - (target_positions()
                                .iter()
                                .cloned()
                                .fold(dec!(0), |acc, x| acc + x.value))) * dec!(-1))
                            .round_dp(0);
                        if strategy.get() == StrategyState::BuySell
                            || !positions.get().is_valid_target_allocation()
                            || !positions.get().all_positions_above_zero() || diff == dec!(0)
                        {
                            view! {
                                {position_total().to_string()}
                                {get_diff_string(dec!(0))}
                                {"".to_string()}
                                {"".to_string()}
                            }
                        } else {
                            view! {
                                {position_total().to_string()}
                                {get_diff_string(diff)}
                                {" = ".to_string()}
                                {target_positions()
                                    .iter()
                                    .cloned()
                                    .fold(dec!(0), |acc, x| acc + x.value)
                                    .round_dp(0)
                                    .to_string()}
                            }
                        }
                    }}
                </span>
            </section>
        </main>
    }
}
