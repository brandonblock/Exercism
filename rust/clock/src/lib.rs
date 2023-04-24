use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let rolled_over = Self::rollover_minutes(Self::rollover_hours(hours), minutes);
        Clock {
            hours: rolled_over.0,
            minutes: rolled_over.1,
        }
    }

    fn rollover_hours(hours: i32) -> i32 {
        if hours < 0 {
            Self::rollover_hours(24 + hours)
        } else if hours >= 24 {
            hours % 24
        } else {
            hours
        }
    }

    fn rollover_minutes(hours: i32, minutes: i32) -> (i32, i32) {
        if minutes < 0 {
            Self::rollover_minutes(Self::rollover_hours(hours - 1), 60 + minutes)
        } else if minutes >= 60 {
            (Self::rollover_hours(hours + minutes / 60), minutes % 60)
        } else {
            (hours, minutes)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let rolled_over = Self::rollover_minutes(self.hours, self.minutes + minutes);
        Self {
            hours: rolled_over.0,
            minutes: rolled_over.1,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
