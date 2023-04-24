use std::time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
//1000000000
pub fn after(start: DateTime) -> DateTime {
    start + Duration::new(1_000_000_000, 0)
}
