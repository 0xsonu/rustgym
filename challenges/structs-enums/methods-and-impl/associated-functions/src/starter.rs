/// A color represented as RGB values.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Creates a new Color from RGB values.
    /// TODO: Return a Color with the given r, g, b values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        todo!()
    }

    /// Creates a red color (255, 0, 0).
    /// TODO: Return a Color representing red.
    pub fn red() -> Self {
        todo!()
    }

    /// Creates a green color (0, 255, 0).
    /// TODO: Return a Color representing green.
    pub fn green() -> Self {
        todo!()
    }

    /// Creates a blue color (0, 0, 255).
    /// TODO: Return a Color representing blue.
    pub fn blue() -> Self {
        todo!()
    }

    /// Returns the hex representation of the color.
    /// TODO: Use format! with {:02X} to create a hex string like "#FF0000".
    pub fn to_hex(&self) -> String {
        todo!()
    }
}
