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
                .filter(|asset| asset.target_allocation != dec!(0))
                .min_by_key(|asset| {
                    (asset.allocation - asset.target_allocation) * polarity
                        / asset.target_allocation
                })
                .cloned()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{PositionInputState, PositionsDataStore, StrategyState};

    #[test]
    fn get_target_assets_negative_position_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(-300),
                    target_allocation: dec!(0.7),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.2),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(200),
                    target_allocation: dec!(0.1),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(-300));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(300));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(200));
    }

    #[test]
    fn get_target_assets_zero_position_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.7),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.2),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(200),
                    target_allocation: dec!(0.1),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(0));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(300));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(200));
    }

    #[test]
    fn get_target_assets_negative_allocation_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(1.4),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.6),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(200),
                    target_allocation: dec!(-1),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(300));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(300));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(200));
    }

    #[test]
    fn get_target_assets_target_allocations_dont_sum_up_to_one_hundred_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.7),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.2),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(200),
                    target_allocation: dec!(0),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(300));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(300));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(200));
    }

    #[test]
    fn get_target_assets_target_allocations_above_one_hundred_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.7),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.2),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(200),
                    target_allocation: dec!(0.2),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(300));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(300));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(200));
    }

    #[test]
    fn get_target_assets_buy_strategy_works() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.7),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.2),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(200),
                    target_allocation: dec!(0.1),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(1400));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(400));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(200));
    }

    #[test]
    fn get_target_assets_sell_strategy_works() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(700),
                    target_allocation: dec!(0.7),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.2),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.1),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::Sell, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(700));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(200));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(100));
    }

    #[test]
    fn get_target_assets_buy_sell_strategy_works() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = PositionsDataStore {
            rows: vec![
                PositionInputState {
                    id: first_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.7),
                },
                PositionInputState {
                    id: second_id,
                    name: "".to_string(),
                    current_position: dec!(300),
                    target_allocation: dec!(0.2),
                },
                PositionInputState {
                    id: third_id,
                    name: "".to_string(),
                    current_position: dec!(200),
                    target_allocation: dec!(0.1),
                },
            ],
        };

        // Act
        let result = get_target_assets(StrategyState::BuySell, positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, first_id);
        assert_eq!(result[0].value, dec!(560));
        assert_eq!(result[1].id, second_id);
        assert_eq!(result[1].value, dec!(160));
        assert_eq!(result[2].id, third_id);
        assert_eq!(result[2].value, dec!(80));
    }
}
