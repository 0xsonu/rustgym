/// A color represented as RGB values.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Creates a new Color from RGB values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Creates a red color (255, 0, 0).
    pub fn red() -> Self {
        Color { r: 255, g: 0, b: 0 }
    }

    /// Creates a green color (0, 255, 0).
    pub fn green() -> Self {
        Color { r: 0, g: 255, b: 0 }
    }

    /// Creates a blue color (0, 0, 255).
    pub fn blue() -> Self {
        Color { r: 0, g: 0, b: 255 }
    }

    /// Returns the hex representation of the color.
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
