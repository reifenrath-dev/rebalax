use crate::i18n::use_i18n;
use crate::types::portfolio::Portfolio;
use crate::types::position::Position;
use leptos_i18n::t_string;
use reactive_stores::Store;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Store, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfoliosDataStore {
    pub selected_portfolio: Uuid,
    pub portfolios: BTreeMap<Uuid, Portfolio>,
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
