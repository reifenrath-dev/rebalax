use std::collections::HashMap;

use reactive_stores::Store;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Store, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfoliosDataStore {
    pub selected_portfolio: Uuid,
    pub portfolios: HashMap<Uuid, Portfolio>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Portfolio {
    pub name: String,
    pub positions: HashMap<Uuid, Position>,
}

impl Default for PortfoliosDataStore {
    fn default() -> Self {
        let id = Uuid::now_v7();
        Self {
            selected_portfolio: id,
            portfolios: HashMap::from([(
                id,
                Portfolio {
                    name: "My First Portfolio".to_string(),
                    positions: HashMap::from([
                        (
                            Uuid::now_v7(),
                            Position {
                                name: "Position 1".to_string(),
                                current_position: dec!(0),
                                target_allocation: dec!(0.70),
                            },
                        ),
                        (
                            Uuid::now_v7(),
                            Position {
                                name: "Position 2".to_string(),
                                current_position: dec!(0),
                                target_allocation: dec!(0.30),
                            },
                        ),
                    ]),
                },
            )]),
        }
    }
}

impl PortfoliosDataStore {
    pub fn get_current(&self) -> &Portfolio {
        &self.portfolios[&self.selected_portfolio]
    }
    pub fn set_portfolio_name(&mut self, portfolio_name: String) {
        self.portfolios
            .get_mut(&self.selected_portfolio)
            .unwrap()
            .name = portfolio_name;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Position {
    pub name: String,
    pub current_position: Decimal,
    pub target_allocation: Decimal,
}

impl Portfolio {
    pub fn is_valid_target_allocation(&self) -> bool {
        self.positions
            .iter()
            .map(|(_, x)| x.target_allocation)
            .sum::<Decimal>()
            == dec!(1)
            && !self
                .positions
                .iter()
                .any(|(_, row)| row.target_allocation.is_sign_negative())
    }
    pub fn total(&self) -> Decimal {
        self.positions
            .iter()
            .fold(dec!(0), |acc, (_, row)| acc + row.current_position)
    }
    pub fn all_positions_above_zero(&self) -> bool {
        self.positions
            .iter()
            .all(|(_, x)| !x.current_position.is_zero() && x.current_position.is_sign_positive())
    }
    pub fn allocation_for(&self, id: &Uuid) -> Decimal {
        if self.total() == dec!(0) {
            dec!(0)
        } else {
            self.positions
                .iter()
                .find(|(x, _)| *x == id)
                .unwrap()
                .1
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
        let sut = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 1".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.70),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 2".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.28),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 3".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.0001),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 4".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.0199),
                    },
                ),
            ]),
        };

        assert!(sut.is_valid_target_allocation());
    }

    #[test]
    fn is_valid_target_allocation_exceeds_100percent() {
        let sut = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 1".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.70),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 2".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.28),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 3".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.0002),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 4".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.0199),
                    },
                ),
            ]),
        };

        assert!(!sut.is_valid_target_allocation());
    }

    #[test]
    fn is_valid_target_allocation_not_100percent() {
        let sut = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 1".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.70),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 2".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.28),
                    },
                ),
            ]),
        };

        assert!(!sut.is_valid_target_allocation());
    }

    #[test]
    fn is_valid_target_allocation_negative_position() {
        let sut = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: HashMap::from([
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 1".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.70),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 2".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(0.30),
                    },
                ),
                (
                    Uuid::now_v7(),
                    Position {
                        name: "Position 3".to_string(),
                        current_position: dec!(0),
                        target_allocation: dec!(-0.10),
                    },
                ),
            ]),
        };

        assert!(!sut.is_valid_target_allocation());
    }
}
