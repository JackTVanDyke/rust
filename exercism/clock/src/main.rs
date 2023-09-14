// Implement a clock that handles times without dates.
// You should be able to add and subtract minutes to it.
// Two clocks that represent the same time should be equal to each other.

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = minutes + hours*60;
        let mut hours = minutes/60;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }

        if hours < 0 {
            while hours < 0 {
                hours += 24;
            }
        } else if hours >= 24 {
            while hours >= 24 {
                hours -= 24;
            }
        }

        let new_clock = Clock {
            hours: hours,
            minutes: minutes
        };

        new_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let updated_clock = Self::new(self.hours, self.minutes+minutes);
        updated_clock
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:0>2}:{:0>2})", self.hours, self.minutes)
    }
}


fn main() {
    println!("{}", -110 / 60);
    println!("{}", -110 % 60);
}
