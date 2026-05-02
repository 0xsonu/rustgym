/// Formats an item name and price into a display string.
pub fn format_price(item: &str, price: f64) -> String {
    format!("{}: ${:.2}", item, price)
}
