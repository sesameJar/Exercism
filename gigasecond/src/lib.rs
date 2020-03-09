use chrono::{DateTime, Duration, Utc};
use std::ops::Add;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let sec_dur = Duration::seconds(1e9 as i64);
    start.add(sec_dur)
}
