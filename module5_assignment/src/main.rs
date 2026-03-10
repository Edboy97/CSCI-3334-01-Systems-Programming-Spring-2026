mod bank_account;
use bank_account::BankAccount;

fn main() {
    let mut my_account = BankAccount::new(1000.0);
    println!("Initial balance: ${}", my_account.balance());
    my_account.deposit(700.0);
    println!("After deposit: ${}", my_account.balance());
    my_account.withdraw(300.0);
    println!("After withdrawal: ${}", my_account.balance());
    my_account.withdraw(5000.0);
    println!("After failed overdraw: ${}", my_account.balance());
    my_account.apply_interest(0.04);
    println!("After 4% interest: ${}", my_account.balance());
}