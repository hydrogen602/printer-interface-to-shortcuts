#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl Time {
    pub fn from_seconds(seconds: i64) -> Option<Self> {
        if seconds < 0 {
            return None;
        }
        let hours = (seconds / 3600).try_into().ok()?;
        let minutes = ((seconds % 3600) / 60).try_into().unwrap();
        let seconds = (seconds % 60).try_into().unwrap();
        Some(Self {
            hours,
            minutes,
            seconds,
        })
    }

    pub fn to_human_readable_briefly(&self) -> String {
        if self.hours > 1 {
            // approx minutes
            let min = (self.minutes / 10) * 10;
            format!("{} hours and {} minutes", self.hours, min)
        } else if self.hours == 1 {
            format!("{} hour and {} minutes", self.hours, self.minutes)
        } else {
            format!("{} minutes", self.minutes)
        }
    }
}
