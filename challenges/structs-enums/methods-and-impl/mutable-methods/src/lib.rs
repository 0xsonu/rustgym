/// A counter that can be incremented and decremented.
pub struct Counter {
    pub value: i32,
}

impl Counter {
    /// Creates a new Counter starting at 0.
    pub fn new() -> Self {
        Counter { value: 0 }
    }

    /// Increments the counter by 1.
    pub fn increment(&mut self) {
        self.value += 1;
    }

    /// Decrements the counter by 1.
    pub fn decrement(&mut self) {
        self.value -= 1;
    }

    /// Resets the counter to 0.
    pub fn reset(&mut self) {
        self.value = 0;
    }

    /// Returns the current value.
    pub fn get(&self) -> i32 {
        self.value
    }
}
