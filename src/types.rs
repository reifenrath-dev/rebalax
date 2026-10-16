use reactive_stores::Store;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Store, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfolioDataStore {
    pub id: Uuid,
    pub name: String,
    #[store(key: Uuid = |row| row.id)]
    pub positions: Vec<PositionInputState>,
}

impl Default for PortfolioDataStore {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: "My First Portfolio".to_string(),
            positions: vec![
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 1".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.70),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 2".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.30),
                },
            ],
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PositionInputState {
    pub id: Uuid,
    pub name: String,
    pub current_position: Decimal,
    pub target_allocation: Decimal,
}

impl PortfolioDataStore {
    pub fn is_valid_target_allocation(&self) -> bool {
        self.positions
            .iter()
            .map(|x| x.target_allocation)
            .sum::<Decimal>()
            == dec!(1)
            && !self
                .positions
                .iter()
                .any(|row| row.target_allocation.is_sign_negative())
    }
    pub fn total(&self) -> Decimal {
        self.positions
            .iter()
            .fold(dec!(0), |acc, row| acc + row.current_position)
    }
    pub fn all_positions_above_zero(&self) -> bool {
        self.positions
            .iter()
            .all(|x| !x.current_position.is_zero() && x.current_position.is_sign_positive())
    }
    pub fn allocation_for(&self, id: Uuid) -> Decimal {
        if self.total() == dec!(0) {
            dec!(0)
        } else {
            self.positions
                .iter()
                .find(|x| x.id == id)
                .unwrap()
                .current_position
                / self.total()
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    EnumString,
    Display,
    EnumIter,
    Hash,
    Eq,
    Default,
)]
pub enum StrategyState {
    /// Readjust portfolio allocation only by buying additional funds
    #[default]
    Buy,
    /// Readjust portfolio allocation without adding funds by shifting funds inside the portfolio
    Reallocate,
    /// Readjust portfolio allocation only by selling funds
    Sell,
}

#[derive(Clone)]
pub struct TargetPosition {
    pub id: Uuid,
    pub value: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_target_allocation_true() {
        let sut = PortfolioDataStore {
            id: Uuid::now_v7(),
            name: "Portfolio 1".to_string(),
            positions: vec![
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 1".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.70),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 2".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.28),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 3".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.0001),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 4".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.0199),
                },
            ],
        };

        assert!(sut.is_valid_target_allocation());
    }

    #[test]
    fn is_valid_target_allocation_exceeds_100percent() {
        let sut = PortfolioDataStore {
            id: Uuid::now_v7(),
            name: "Portfolio 1".to_string(),
            positions: vec![
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 1".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.70),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 2".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.28),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 3".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.0002),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 4".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.0199),
                },
            ],
        };

        assert!(!sut.is_valid_target_allocation());
    }

    #[test]
    fn is_valid_target_allocation_not_100percent() {
        let sut = PortfolioDataStore {
            id: Uuid::now_v7(),
            name: "Portfolio 1".to_string(),
            positions: vec![
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 1".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.70),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 2".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.28),
                },
            ],
        };

        assert!(!sut.is_valid_target_allocation());
    }

    #[test]
    fn is_valid_target_allocation_negative_position() {
        let sut = PortfolioDataStore {
            id: Uuid::now_v7(),
            name: "Portfolio 1".to_string(),
            positions: vec![
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 1".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.70),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 2".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(0.30),
                },
                PositionInputState {
                    id: Uuid::now_v7(),
                    name: "Position 2".to_string(),
                    current_position: dec!(0),
                    target_allocation: dec!(-0.10),
                },
            ],
        };

        assert!(!sut.is_valid_target_allocation());
    }
}
