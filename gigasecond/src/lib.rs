use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

const GIGASECOND: f64 = 1e9;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.checked_add(GIGASECOND.seconds()).unwrap()
}