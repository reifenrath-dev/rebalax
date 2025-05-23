use reactive_stores::Store;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Store, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Data {
    #[store(key: Uuid = |row| row.id.clone())]
    pub rows: Vec<AssetInputState>,
}

impl Default for Data {
    fn default() -> Self {
        Self { rows: vec![] }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetInputState {
    pub id: Uuid,
    pub name: String,
    pub current_position: Decimal,
    pub target_allocation: Decimal,
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
pub struct TargetAsset {
    pub id: Uuid,
    pub value: Decimal,
}
