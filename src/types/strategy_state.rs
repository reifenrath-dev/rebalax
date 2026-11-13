use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

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
