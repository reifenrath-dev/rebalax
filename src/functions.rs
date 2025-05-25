use crate::types::{PositionsDataStore, StrategyState, TargetPosition};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

#[derive(Clone)]
struct UnbalancedAsset {
    id: Uuid,
    allocation: Decimal,
    target_allocation: Decimal,
    position: Decimal,
}

pub fn get_target_assets(
    strategy: StrategyState,
    positions_store: PositionsDataStore,
) -> Vec<TargetPosition> {
    if !positions_store.is_valid_target_allocation() || !positions_store.all_positions_above_zero()
    {
        return positions_store
            .rows
            .iter()
            .cloned()
            .map(|x| TargetPosition {
                id: x.id,
                value: x.current_position,
            })
            .collect::<Vec<TargetPosition>>();
    }
    let position_total = positions_store
        .rows
        .iter()
        .cloned()
        .map(|x| x.current_position)
        .sum::<Decimal>();

    match strategy {
        z @ StrategyState::Buy | z @ StrategyState::Sell => {
            let is_buy = z == StrategyState::Buy;
            let polarity = if is_buy { dec!(-1) } else { dec!(1) };

            let assets: Vec<UnbalancedAsset> = positions_store
                .rows
                .iter()
                .cloned()
                .map(|x| UnbalancedAsset {
                    id: x.id,
                    allocation: x.current_position / position_total,
                    target_allocation: x.target_allocation,
                    position: x.current_position,
                })
                .collect();

            let highest_deviation = assets
                .iter()
                .cloned()
                .filter(|asset| asset.target_allocation != dec!(0))
                .min_by_key(|asset| {
                    (asset.allocation - asset.target_allocation) * polarity
                        / asset.target_allocation
                })
                .unwrap();

            let factor = highest_deviation.position / highest_deviation.target_allocation;

            assets
                .iter()
                .cloned()
                .map(|asset| TargetPosition {
                    id: asset.id,
                    value: asset.target_allocation * factor,
                })
                .collect::<Vec<TargetPosition>>()
        }
        StrategyState::BuySell => positions_store
            .rows
            .iter()
            .cloned()
            .map(|position| TargetPosition {
                id: position.id,
                value: position.target_allocation * position_total,
            })
            .collect(),
    }
}
