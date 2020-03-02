use chrono::{DateTime, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let ts = start.timestamp_millis() + 1e12 as i64;

    Utc.timestamp_millis(ts)
}
