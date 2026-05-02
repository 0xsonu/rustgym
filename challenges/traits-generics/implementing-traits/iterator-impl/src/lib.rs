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

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}
