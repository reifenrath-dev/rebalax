use crate::types::position::Position;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Portfolio {
    pub name: String,
    pub positions: BTreeMap<Uuid, Position>,
}

impl Portfolio {
    pub fn is_valid_target_allocation(&self) -> bool {
        self.positions
            .values()
            .map(|x| x.target_allocation)
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

impl Default for Portfolio {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            positions: BTreeMap::from([(Uuid::now_v7(), Default::default())]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::portfolio::Portfolio;
    use std::collections::BTreeMap;
    use uuid::Uuid;

    #[test]
    fn is_valid_target_allocation_true() {
        let sut = Portfolio {
            name: "Portfolio 1".to_string(),
            positions: BTreeMap::from([
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
            positions: BTreeMap::from([
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
            positions: BTreeMap::from([
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
            positions: BTreeMap::from([
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
