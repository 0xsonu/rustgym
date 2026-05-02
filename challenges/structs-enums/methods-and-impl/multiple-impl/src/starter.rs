/// A bank account with a balance.
pub struct BankAccount {
    pub owner: String,
    pub balance: f64,
}

impl BankAccount {
    /// Creates a new account with zero balance.
    /// TODO: Return a BankAccount with the given owner and 0.0 balance.
    pub fn new(owner: &str) -> Self {
        todo!()
    }

    /// Deposits money into the account.
    /// TODO: Add amount to balance (only if amount > 0).
    pub fn deposit(&mut self, amount: f64) {
        todo!()
    }

    /// Withdraws money from the account. Returns true if successful.
    /// TODO: Subtract amount from balance if amount > 0 and balance >= amount.
    pub fn withdraw(&mut self, amount: f64) -> bool {
        todo!()
    }

    /// Returns a summary string.
    /// TODO: Return "owner: $balance" formatted with 2 decimal places.
    pub fn summary(&self) -> String {
        todo!()
    }
}
