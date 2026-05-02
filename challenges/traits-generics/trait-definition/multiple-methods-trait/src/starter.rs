/// A trait for shapes that have area and perimeter.
/// TODO: Define the trait with area(), perimeter(), and description() methods.
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn description(&self) -> String;
}

pub struct Square {
    pub side: f64,
}

/// TODO: Implement Shape for Square.
impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

/// TODO: Implement Shape for Triangle.
impl Shape for Triangle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }
}
