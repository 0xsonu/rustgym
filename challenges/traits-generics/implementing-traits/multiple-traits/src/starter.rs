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

/// TODO: Implement Display for Duration.
/// Format: "HH:MM:SS" (e.g., "01:30:00" for 5400 seconds)
impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// TODO: Implement PartialEq for Duration.
/// Two durations are equal if they have the same number of seconds.
impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// TODO: Implement PartialOrd for Duration.
/// Compare by total seconds.
impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}
