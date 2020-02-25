use chrono::{DateTime, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let mut ts = start.timestamp();
    ts += 10i64.pow(9);
    let dt = Utc.timestamp(ts, 0);
    dt
}
