use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Position {
    pub name: String,
    pub current_position: Decimal,
    pub target_allocation: Decimal,
}
