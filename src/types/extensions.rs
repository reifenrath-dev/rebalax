use leptos::ev::Event;
use leptos::prelude::event_target_value;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

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
