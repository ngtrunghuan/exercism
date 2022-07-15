use std::fmt;
#[derive(PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn fmt_clock(clock: &Clock, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:02}:{:02}", clock.hours, clock.minutes)
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt_clock(self, f);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt_clock(self, f);
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: 0,
            minutes: 0,
        }
        .add_hours(hours)
        .add_minutes(minutes)
    }

    fn add_hours(&self, hours: i32) -> Self {
        let new_hours = {
            if self.hours + hours >= 24 {
                (self.hours + hours) % 24
            } else if self.hours + hours < 0 {
                if (self.hours + hours) % 24 == 0 {
                    0
                } else {
                    24 + (self.hours + hours) % 24
                }
            } else {
                self.hours + hours
            }
        };
        Clock {
            hours: new_hours,
            minutes: self.minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes: i32;
        let hours: i32;
        if self.minutes + minutes >= 60 {
            new_minutes = (self.minutes + minutes) % 60;
            hours = (self.minutes + minutes) / 60;
        } else if self.minutes + minutes < 0 {
            if ((self.minutes + minutes) % 60) == 0 {
                new_minutes = 0;
                hours = (self.minutes + minutes) / 60;
            } else {
                new_minutes = 60 + (self.minutes + minutes) % 60;
                hours = (self.minutes + minutes) / 60 - 1
            }
        } else {
            new_minutes = self.minutes + minutes;
            hours = 0
        }
        Clock {
            hours: self.hours,
            minutes: new_minutes,
        }
        .add_hours(hours)
    }
}
