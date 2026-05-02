/// A trait for shapes that have area and perimeter.
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn description(&self) -> String;
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

    fn description(&self) -> String {
        format!("Square with side {}", self.side)
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }

    fn description(&self) -> String {
        format!("Triangle with base {} and height {}", self.base, self.height)
    }
}
