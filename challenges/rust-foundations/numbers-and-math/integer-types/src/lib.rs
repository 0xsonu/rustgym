/// Returns the maximum values of u8, i8, and u16 as u32 values.
pub fn integer_limits() -> (u32, u32, u32) {
    (u8::MAX as u32, i8::MAX as u32, u16::MAX as u32)
}
