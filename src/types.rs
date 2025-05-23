use reactive_stores::Store;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Store, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionsDataStore {
    #[store(key: Uuid = |row| row.id.clone())]
    pub rows: Vec<PositionInputState>,
}

impl Default for PositionsDataStore {
    fn default() -> Self {
        Self { rows: vec![] }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PositionInputState {
    pub id: Uuid,
    pub name: String,
    pub current_position: Decimal,
    pub target_allocation: Decimal,
}

impl PositionsDataStore {
    pub fn is_valid_target_allocation(&self) -> bool {
        self.rows
            .iter()
            .cloned()
            .map(|x| x.target_allocation)
            .sum::<Decimal>()
            == dec!(1)
    }
    pub fn total(&self) -> Decimal {
        self.rows.iter().fold(dec!(0.0), |acc, row| {acc + row.current_position})
    }
    pub fn allocation_for(&self, id: Uuid) -> Decimal {
        if self.total() == dec!(0) {
            dec!(0)
        } else {
            self.rows.iter().find(|x| x.id == id).unwrap().current_position / self.total()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, EnumString, Display, EnumIter)]
pub enum StrategyState {
    Buy,
    BuySell,
    Sell,
}

impl Default for StrategyState {
    fn default() -> Self {
        StrategyState::Buy
    }
}

#[derive(Clone)]
pub struct TargetPosition {
    pub id: Uuid,
    pub value: Decimal,
}
