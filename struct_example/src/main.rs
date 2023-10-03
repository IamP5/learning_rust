#[derive(Debug)]
struct Account {
    name: String,
    balance: f64,
}

struct Transaction<'a> {
    from: &'a mut Account,
    to: &'a mut Account,
    amount: f64,
}

impl Account {
    fn new(name: String, balance: f64) -> Self {
        Self { name, balance }
    }

    fn add_balance(&mut self, value: f64) {
        self.balance += value;
    }

    fn remove_balance(&mut self, value: f64) {
        self.balance -= value;
    }
}

impl<'a> Transaction<'a> {
    fn new(from: &'a mut Account, to: &'a mut Account, amount: f64) -> Self {
        Self { from, to, amount }
    }

    fn execute(&mut self) {
        self.from.remove_balance(self.amount);
        self.to.add_balance(self.amount);
    }
}

fn main() {
    let mut a1 = Account::new("Tuba".to_string(), 2500.00);
    let mut a2 = Account::new("Jonsons".to_string(), 1000.00);

    {
        let mut transaction = Transaction::new(&mut a1, &mut a2, 1000.00);
        transaction.execute();
    }

    println!("Updated {}'s account -> {}", a1.name, a1.balance);
    println!("Updated {}'s account -> {}", a2.name, a2.balance);
}
