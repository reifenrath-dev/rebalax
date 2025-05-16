use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use reactive_stores::{Store};

#[derive(Store, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Data {
    #[store(key: usize = |row| row.id.clone())]
    rows: Vec<AssetInputState>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            rows: vec![],
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct AssetInputState {
    id: usize,
    name: String,
    current_position: Decimal,
    target_allocation: Decimal,
}

#[derive(Clone)]
struct UnbalancedAsset {
    allocation: Decimal,
    target_allocation: Decimal,
    position: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, EnumString, Display, EnumIter)]
enum StrategyState {
    Buy,
    BuySell,
    Sell,
}

impl Default for StrategyState  {
    fn default() -> Self {
        StrategyState::Buy
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (strategy, set_strategy, _) = use_local_storage::<StrategyState, JsonSerdeCodec>("strategy-state");

    let (positions, set_positions, _) =
        use_local_storage::<Data, JsonSerdeCodec>("asset-state");
    if positions.get().rows.len() == 0 {
        set_positions.set(Data {
            rows: vec![AssetInputState {
                id: 0,
                name: "Position 1".to_string(),
                current_position: dec!(0),
                target_allocation: dec!(0),
            }, AssetInputState {
                id: 1,
                name: "Position 2".to_string(),
                current_position: dec!(0),
                target_allocation: dec!(0),
            }]
        });
    }

    // Value Functions
    let position_total = move || positions.get().rows.iter().cloned().map(|x| x.current_position).sum::<Decimal>();
    let allocation = move |position: Decimal| if position_total() == dec!(0) { dec!(0) } else { position / position_total() };

    let target_positions = move || {
        if positions.get().rows.iter().cloned().map(|x| x.target_allocation).sum::<Decimal>() != dec!(1)
        {
            return positions.get().rows.iter().cloned().map(|_| dec!(0)).collect();
        }

        return match strategy.get() {
            z @ StrategyState::Buy | z @ StrategyState::Sell => {
                let is_buy = z == StrategyState::Buy;
                let polarity = if is_buy { dec!(-1) } else { dec!(1) };

                let assets: Vec<UnbalancedAsset> = positions.get().rows.iter().cloned()
                    .map(|x| UnbalancedAsset {
                        allocation: allocation(x.current_position),
                        target_allocation: x.target_allocation,
                        position: x.current_position,
                    })
                    .collect();

                let highest_deviation = assets.iter().cloned()
                    .filter(|asset| asset.target_allocation != dec!(0))
                    .min_by_key(|asset| (asset.allocation - asset.target_allocation) * polarity / asset.target_allocation)
                    .unwrap();

                let factor = highest_deviation.position / highest_deviation.target_allocation;

                assets.iter().cloned()
                    .map(|asset| asset.target_allocation * factor)
                    .collect::<Vec<Decimal>>()
            }
            StrategyState::BuySell => {
                let total = position_total();
                positions.get().rows.iter().cloned().map(|position| position.target_allocation * total).collect::<Vec<Decimal>>()
            }
        };
    };

    let get_diff_string = |diff: Decimal| {
        if diff.is_zero() {
            view!{ <span class="zero">{"".to_string()}</span> }
        } else {
            if diff.is_sign_positive() {
                view!{ <span class="positive">{format!(" (+{})", diff)}</span> }
            } else {
                view!{ <span class="negative">{format!(" ({})", diff)}</span> }
            }
        }
    };

    view! {
        <main class="container">
            <div class="strategy-section">
                <b class="strategy-title">Strategy:</b>
                <div class="strategy-options">
                    {StrategyState::iter().map(|stra| view! {
                        <input
                            type="radio"
                            name="strategy"
                            id={ format!("strategy-{}", stra) }
                            value={stra.to_string()}
                            checked=move || strategy.get() == stra
                            on:change=move |_| set_strategy.set(stra)
                        />
                        <label for={ format!("strategy-{}", stra) }>
                            {stra.to_string()}
                        </label>
                    }).collect_view()}
                </div>
            </div>

            <table>
                <tr>
                    <th>Name</th>
                    <th>Position</th>
                    <th>%</th>
                </tr>

                <For
                    each=move || positions.get().rows.clone()
                    key=|row| row.id.clone()
                    children=move |position| {
                        view!{
                            <tr>
                                <td colspan=3 class="title">
                                    <input
                                        type="text"
                                        value={position.name}
                                        on:input={ move |ev| {
                                            let mut new_positions = positions.get().rows;
                                            new_positions[position.id].name = event_target_value(&ev).parse().unwrap();
                                            set_positions.set( Data { rows: new_positions })}
                                        }
                                    />
                                </td>
                            </tr>
                            <tr class="current">
                                <td>Current</td>
                                <td class="number">
                                    <input
                                        id={ format!("{}-position-input", position.id) }
                                        placeholder="..."
                                        type="number"
                                        value={position.current_position.to_string()}
                                        on:input={ move |ev| {
                                            let mut new_positions = positions.get().rows;
                                            new_positions[position.id].current_position = event_target_value(&ev).parse::<Decimal>().unwrap();
                                            set_positions.set( Data { rows: new_positions })}
                                        }
                                    />
                                </td>
                                <td class="number"><div class="number">{ move || format!("{}", (allocation(position.current_position) * dec!(100)).round_dp(2).to_string()) }</div></td>
                            </tr>
                            <tr class="target">
                                <td>Target</td>
                                <td class="number">
                                    <div class="number">{ move ||
                                        target_positions()[position.id].round_dp(0).to_string()
                                    }
                                    { move ||
                                        get_diff_string((target_positions()[position.id] - positions.get().rows[position.id].current_position).round_dp(0))
                                    }
                                    </div>
                                </td>
                                <td class="number">
                                    <input
                                        id={ format!("{}-target-input", position.id) }
                                        placeholder="..."
                                        type="number"
                                        value={(position.target_allocation * dec!(100)).to_string()}
                                        on:input={ move |ev| {
                                            let mut new_positions = positions.get().rows;
                                            new_positions[position.id].target_allocation = event_target_value(&ev).parse::<Decimal>().unwrap()  / dec!(100);
                                            set_positions.set(Data {rows: new_positions})}
                                        }
                                    />
                                </td>
                            </tr>
                    }}
                />
            </table>

            <div class="add-remove-section">
                <button class="minus" on:click={move |_| {
                    set_positions.update(|value| {
                        value.rows.pop();
                        *value = value.clone()
                    })
                }}>-</button>
                <button class="plus" on:click={move |_| {
                    let new_id = positions.get().rows.len();
                    set_positions.update(|value| *value = Data { rows: value.rows.iter().cloned().chain([AssetInputState {
                        id: new_id,
                        name: format!("Position {}", new_id+1),
                        current_position: dec!(0),
                        target_allocation: dec!(0),}]).collect()})
                }}>+</button>
            </div>

            <table>
                <tr>
                    <td colspan="3" class="title">Total</td>
                </tr>
                <tr class="current">
                    <td>Current</td>
                    <td class="number"><div class="number">{ move || position_total().to_string() }</div></td>
                    <td class="number">{ move || format!("{}",
                        ((positions.get().rows.iter().cloned()
                            .map(|position| allocation(position.current_position))
                            .sum::<Decimal>()) * dec!(100))
                        .round_dp(2))
                    }</td>
                </tr>
                <tr class="target">
                    <td>Target</td>
                    <td class="number"><div class="number">{ move || format!("{}",
                        (target_positions().iter().cloned().sum::<Decimal>()).round_dp(0).to_string())
                    }
                    { move ||
                        get_diff_string(((position_total() - (target_positions().iter().cloned().sum::<Decimal>())) * dec!(-1)).round_dp(0))
                    }</div></td>
                    <td class="number">{ move || format!("{}",
                        (positions.get().rows.iter().cloned()
                            .map(|position| position.target_allocation)
                            .sum::<Decimal>())
                        * dec!(100).round_dp(2))
                    }</td>
                </tr>
            </table>
        </main>
    }
}