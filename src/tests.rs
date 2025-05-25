#[cfg(test)]
mod tests {
    use crate::functions::get_target_assets;
    use crate::types::{PositionInputState, PositionsDataStore, StrategyState};
    use rust_decimal_macros::dec;
    use uuid::Uuid;

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
