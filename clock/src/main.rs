use std::fmt::{Display,Formatter, Result};

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{}", self.hours, self.minutes)
    }
}

impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        let additional_hours = minutes / 60 as i32;
        let mut hours = additional_hours + hours;
        hours = if hours > 23 {hours % 24 } else if hours < 0 { hours % -24} else {hours};
        
        Clock {hours, minutes: minutes % 60 }
    }

    fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let mut new_minutes = self.minutes + minutes;

        if new_minutes > 59 {
            hours = new_minutes / 60 as i32 + hours;
            new_minutes = new_minutes % 60;
        }

        hours = if hours > 23 {hours % 24} else {hours};
        Self {hours, minutes: new_minutes}
    }

    fn subtract_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let mut new_minutes = self.minutes - minutes;
        if new_minutes < 0 {
            hours = hours - new_minutes / 60 as i32;
            new_minutes = new_minutes % 60;
        }

        hours = if hours < 0 {24 - hours} else {hours};

        Self {hours, minutes: new_minutes}
    }
}

fn main() {
    let clock = Clock::new(5,35);
    println!("{clock}");
}
