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

    pub fn to_human_readable_briefly(self) -> String {
        // if self.hours > 0, then round minutes to 10s
        match (self.hours, self.minutes) {
            (1, 1) => "1 hour and 1 minute".to_string(),
            (1, m) => format!("1 hour and {} minutes", (m / 10) * 10),
            (0, 1) => "1 minute".to_string(),
            (0, m) => format!("{} minutes", m),
            (h, 1) => format!("{} hours and 1 minute", h),
            (h, m) => format!("{} hours and {} minutes", h, (m / 10) * 10),
        }
    }
}
