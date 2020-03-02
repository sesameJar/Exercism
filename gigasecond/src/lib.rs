use chrono::{DateTime, FixedOffset, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let mut ts = start.timestamp_millis() + 1e12 as i64;
    // ts += 10i64.pow(9);
    // Utc.timestamp(ts, 0)
    
    Utc.timestamp_millis(ts)
}
