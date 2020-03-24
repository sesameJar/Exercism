use std::fmt;

pub struct Clock {
    hours: u32,
    minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = ((hours % 24) + (minutes / 60)) % 24;
        let mut m = minutes % 60;
        if m.is_negative() {
            m += 60;
            h -= 1;
        }
        if h.is_negative() {
            h += 24;
        }
        Clock {
            hours: h as u32,
            minutes: m as u32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_mins = self.minutes as i32 + minutes;
        return Clock::new(self.hours as i32, new_mins);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
