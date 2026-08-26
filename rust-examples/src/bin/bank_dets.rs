/*Bank Account
Create an account with:
owner
balance
Implement deposit() and withdraw().*/

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }
}

fn main() {
    let mut account = BankAccount {
        owner: String::from("Neema"),
        balance: 1000.0,
    };

    println!("Account: {} Initial balance: {}",account.owner, account.balance);

    account.deposit(500.0);
    println!("After deposit: {}", account.balance);

    account.withdraw(200.0);
    println!("After withdrawal: {}", account.balance);

    account.withdraw(2000.0);
}