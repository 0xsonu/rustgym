/// A semantic version number.
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
    }
}

/// TODO: Implement PartialEq for Version.
/// Two versions are equal if all three components match.
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// TODO: Implement PartialOrd for Version.
/// Compare major first, then minor, then patch.
impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}
