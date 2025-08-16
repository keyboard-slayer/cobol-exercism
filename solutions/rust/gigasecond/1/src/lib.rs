use chrono::{Duration, DateTime, Utc};


// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let base: i32 = 10;
    return start + Duration::seconds(base.pow(9).into());
}
