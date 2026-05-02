use std::fmt;

/// A duration in seconds.
pub struct Duration {
    pub seconds: u64,
}

impl Duration {
    pub fn new(seconds: u64) -> Self {
        Duration { seconds }
    }

    pub fn from_minutes(minutes: u64) -> Self {
        Duration { seconds: minutes * 60 }
    }

    pub fn from_hours(hours: u64) -> Self {
        Duration { seconds: hours * 3600 }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.seconds / 3600;
        let minutes = (self.seconds % 3600) / 60;
        let secs = self.seconds % 60;
        write!(f, "{:02}:{:02}:{:02}", hours, minutes, secs)
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.seconds == other.seconds
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.seconds.cmp(&other.seconds))
    }
}
