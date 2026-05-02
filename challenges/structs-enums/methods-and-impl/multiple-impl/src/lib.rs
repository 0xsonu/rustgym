/// A bank account with a balance.
pub struct BankAccount {
    pub owner: String,
    pub balance: f64,
}

impl BankAccount {
    /// Creates a new account with zero balance.
    pub fn new(owner: &str) -> Self {
        BankAccount {
            owner: String::from(owner),
            balance: 0.0,
        }
    }

    /// Deposits money into the account.
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    /// Withdraws money from the account. Returns true if successful.
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!("{}: ${:.2}", self.owner, self.balance)
    }
}
