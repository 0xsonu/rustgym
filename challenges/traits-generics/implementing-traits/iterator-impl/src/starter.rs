/// A counter that counts from 1 to a maximum value.
pub struct Counter {
    max: u32,
    current: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Counter { max, current: 0 }
    }
}

/// TODO: Implement Iterator for Counter.
/// Each call to next() should return Some(next_value) until max is reached, then None.
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
