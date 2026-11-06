use crate::types::{Portfolio, StrategyState};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
struct UnbalancedAsset {
    id: Uuid,
    allocation: Decimal,
    target_allocation: Decimal,
    position: Decimal,
}

/// Calculates target asset allocations based on the selected strategy and current positions.
///
/// This function determines the optimal target positions for assets based on the provided
/// strategy state and current position information. It handles three main strategy cases:
/// Buy, Sell, and Reallocate, each with different allocation logic.
///
/// # Arguments
/// * `strategy` - The current trading strategy state (Buy, Sell, or Reallocate)
/// * `positions_store` - Container holding current position data and target allocations
///
/// # Returns
/// A vector of `TargetPosition` structs containing asset IDs and their calculated target values
///
/// # Logic
/// 1. If position allocations are invalid or any positions are zero, returns current positions
/// 2. For Buy/Sell strategies:
///    - Calculates asset allocations relative to total portfolio value
///    - Identifies the asset with the highest deviation from target allocation
///    - Scales all assets proportionally based on the most deviated asset
/// 3. For Reallocate strategy:
///    - Directly scales target allocations by total portfolio value
///
/// # Notes
/// The function uses decimal arithmetic for precise financial calculations and handles
/// edge cases where target allocations might be zero to avoid division by zero errors.
pub fn get_target_assets(
    strategy: StrategyState,
    positions_store: &Portfolio,
) -> HashMap<Uuid, Decimal> {
    if !positions_store.is_valid_target_allocation() || !positions_store.all_positions_above_zero()
    {
        positions_store
            .positions
            .iter()
            .map(|(id, position)| (*id, position.current_position))
            .collect()
    } else {
        let position_total: Decimal = positions_store
            .positions
            .iter()
            .map(|(_, position)| position.current_position)
            .sum();

        match strategy {
            z @ StrategyState::Buy | z @ StrategyState::Sell => {
                let is_buy = z == StrategyState::Buy;
                let polarity = if is_buy { dec!(-1) } else { dec!(1) };

                let assets: Vec<UnbalancedAsset> = positions_store
                    .positions
                    .iter()
                    .map(|(id, position)| UnbalancedAsset {
                        id: *id,
                        allocation: position.current_position / position_total,
                        target_allocation: position.target_allocation,
                        position: position.current_position,
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
                    .map(|asset| (asset.id, asset.target_allocation * factor))
                    .collect()
            }
            StrategyState::Reallocate => positions_store
                .positions
                .iter()
                .map(|(id, position)| (*id, position.target_allocation * position_total))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Portfolio, Position, StrategyState};
    use std::collections::HashMap;

    #[test]
    fn get_target_assets_negative_position_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(-300),
                        target_allocation: dec!(0.7),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.2),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(200),
                        target_allocation: dec!(0.1),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(-300));
        assert_eq!(result[&second_id], dec!(300));
        assert_eq!(result[&third_id], dec!(200));
    }

    #[test]
    fn get_target_assets_zero_position_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.7),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.2),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(200),
                        target_allocation: dec!(0.1),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(0));
        assert_eq!(result[&second_id], dec!(300));
        assert_eq!(result[&third_id], dec!(200));
    }

    #[test]
    fn get_target_assets_negative_allocation_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(1.4),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.6),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(200),
                        target_allocation: dec!(-1),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(300));
        assert_eq!(result[&second_id], dec!(300));
        assert_eq!(result[&third_id], dec!(200));
    }

    #[test]
    fn get_target_assets_target_allocations_dont_sum_up_to_one_hundred_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.7),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.2),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(200),
                        target_allocation: dec!(0),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(300));
        assert_eq!(result[&second_id], dec!(300));
        assert_eq!(result[&third_id], dec!(200));
    }

    #[test]
    fn get_target_assets_target_allocations_above_one_hundred_mirrors_input() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.7),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.2),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(200),
                        target_allocation: dec!(0.2),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(300));
        assert_eq!(result[&second_id], dec!(300));
        assert_eq!(result[&third_id], dec!(200));
    }

    #[test]
    fn get_target_assets_buy_strategy_works() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.7),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.2),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(200),
                        target_allocation: dec!(0.1),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Buy, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(1400));
        assert_eq!(result[&second_id], dec!(400));
        assert_eq!(result[&third_id], dec!(200));
    }

    #[test]
    fn get_target_assets_sell_strategy_works() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(700),
                        target_allocation: dec!(0.7),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.2),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.1),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Sell, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(700));
        assert_eq!(result[&second_id], dec!(200));
        assert_eq!(result[&third_id], dec!(100));
    }

    #[test]
    fn get_target_assets_buy_sell_strategy_works() {
        // Arrange
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let third_id = Uuid::new_v4();
        let positions_store = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    first_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.7),
                    },
                ),
                (
                    second_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(300),
                        target_allocation: dec!(0.2),
                    },
                ),
                (
                    third_id,
                    Position {
                        name: "".to_string(),
                        current_position: dec!(200),
                        target_allocation: dec!(0.1),
                    },
                ),
            ]),
        };

        // Act
        let result = get_target_assets(StrategyState::Reallocate, &positions_store);

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[&first_id], dec!(560));
        assert_eq!(result[&second_id], dec!(160));
        assert_eq!(result[&third_id], dec!(80));
    }
}
