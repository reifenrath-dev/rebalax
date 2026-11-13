use crate::i18n::use_i18n;
use leptos::ev::Event;
use leptos::prelude::event_target_value;
use leptos_i18n::__private::macros_reexport::t_string;
use reactive_stores::Store;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Store, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfoliosDataStore {
    pub selected_portfolio: Uuid,
    pub portfolios: BTreeMap<Uuid, Portfolio>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Portfolio {
    pub name: String,
    pub positions: BTreeMap<Uuid, Position>,
}

impl Default for PortfoliosDataStore {
    fn default() -> Self {
        let i18n = use_i18n();
        let id = Uuid::now_v7();
        Self {
            selected_portfolio: id,
            portfolios: BTreeMap::from([(
                id,
                Portfolio {
                    name: t_string!(i18n, initial_portfolio).to_string(),
                    positions: BTreeMap::from([
                        (
                            Uuid::now_v7(),
                            Position {
                                name: t_string!(i18n, industry_markets).to_string(),
                                current_position: dec!(0),
                                target_allocation: dec!(0.70),
                            },
                        ),
                        (
                            Uuid::now_v7(),
                            Position {
                                name: t_string!(i18n, emerging_markets).to_string(),
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
    pub fn set_position_name(&mut self, position_id: &Uuid, position_name: String) {
        self.portfolios
            .get_mut(&self.selected_portfolio)
            .unwrap()
            .positions
            .get_mut(position_id)
            .unwrap()
            .name = position_name;
    }
    pub fn set_position_current_value(&mut self, position_id: &Uuid, value: Decimal) {
        self.portfolios
            .get_mut(&self.selected_portfolio)
            .unwrap()
            .positions
            .get_mut(position_id)
            .unwrap()
            .current_position = value;
    }
    pub fn set_position_target_allocation(&mut self, position_id: &Uuid, allocation: Decimal) {
        self.portfolios
            .get_mut(&self.selected_portfolio)
            .unwrap()
            .positions
            .get_mut(position_id)
            .unwrap()
            .target_allocation = allocation;
    }
    pub fn remove_position(&mut self, position_id: &Uuid) {
        self.portfolios
            .get_mut(&self.selected_portfolio)
            .unwrap()
            .positions
            .remove(position_id);
    }
    pub fn add_position(&mut self, position: Position) {
        self.portfolios
            .get_mut(&self.selected_portfolio)
            .unwrap()
            .positions
            .insert(Uuid::now_v7(), position);
    }
    pub fn add_portfolio(&mut self, id: Uuid, portfolio: Portfolio) {
        self.portfolios.insert(id, portfolio);
    }
    pub fn remove_portfolio(&mut self, id: Uuid) {
        self.portfolios.remove(&id);
    }
    pub fn next_or_selected_portfolio(&self) -> Uuid {
        self.portfolios
            .range(self.selected_portfolio..)
            .nth(1)
            .map(|(&k, _)| k)
            .unwrap_or(self.selected_portfolio)
    }
    pub fn previous_or_selected_portfolio(&self) -> Uuid {
        self.portfolios
            .range(..self.selected_portfolio)
            .next_back()
            .map(|(&k, _)| k)
            .unwrap_or(self.selected_portfolio)
    }
    pub fn fallback_portfolio(&self) -> Uuid {
        if self.selected_portfolio == self.previous_or_selected_portfolio() {
            self.next_or_selected_portfolio()
        } else {
            self.previous_or_selected_portfolio()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Position {
    pub name: String,
    pub current_position: Decimal,
    pub target_allocation: Decimal,
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

pub trait DecimalExtensions {
    fn to_percentage(self) -> Decimal;
}
impl DecimalExtensions for Decimal {
    fn to_percentage(self) -> Decimal {
        (self * dec!(100)).round_dp(2)
    }
}
pub trait EventExtensions {
    fn to_decimal_or_zero(self) -> Decimal;
}
impl EventExtensions for Event {
    fn to_decimal_or_zero(self) -> Decimal {
        event_target_value(&self).parse().unwrap_or(dec!(0))
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

#[cfg(test)]
mod tests {
    use super::*;

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
