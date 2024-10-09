use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MAX_MINUTES: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let t = fix(hours, minutes);
        Clock {
            hours: t.0,
            minutes: t.1,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

fn fix(hours: i32, minutes: i32) -> (i32, i32) {
    let mut time = (hours * 60 + minutes) % MAX_MINUTES;
    if time < 0 {
        time += MAX_MINUTES;
    }
    ((time / 60) % 24, time % 60)
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
