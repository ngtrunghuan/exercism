use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours + (self.minutes + minutes) / 60,
            minutes: (self.minutes + minutes) % 60,
        }
    }
}
