use multiple_impl::BankAccount;

#[test]
fn test_new_account() {
    let acc = BankAccount::new("Alice");
    assert_eq!(acc.owner, "Alice");
    assert_eq!(acc.balance, 0.0);
}

#[test]
fn test_deposit() {
    let mut acc = BankAccount::new("Bob");
    acc.deposit(100.0);
    assert_eq!(acc.balance, 100.0);
}

#[test]
fn test_deposit_negative_ignored() {
    let mut acc = BankAccount::new("Bob");
    acc.deposit(-50.0);
    assert_eq!(acc.balance, 0.0);
}

#[test]
fn test_withdraw_success() {
    let mut acc = BankAccount::new("Charlie");
    acc.deposit(200.0);
    assert!(acc.withdraw(50.0));
    assert_eq!(acc.balance, 150.0);
}

#[test]
fn test_withdraw_insufficient() {
    let mut acc = BankAccount::new("Dave");
    acc.deposit(30.0);
    assert!(!acc.withdraw(50.0));
    assert_eq!(acc.balance, 30.0);
}

#[test]
fn test_summary() {
    let mut acc = BankAccount::new("Eve");
    acc.deposit(1234.5);
    assert_eq!(acc.summary(), "Eve: $1234.50");
}
