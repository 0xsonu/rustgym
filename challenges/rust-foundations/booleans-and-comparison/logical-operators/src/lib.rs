/// Returns true if the person is 18+ and has a ticket.
pub fn can_enter(age: u32, has_ticket: bool) -> bool {
    age >= 18 && has_ticket
}
