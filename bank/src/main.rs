#[derive(Debug)]

struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(&account);
    }

    // Add account
}

#[derive(Debug)]
struct Account {
    id: i32,
    holder: String,
    balance: u32,
}

impl Account {
    fn new(id: i32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: u32) {
        self.balance -= amount;
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Oluwagbemiga"));

    bank.add_account(&account);
    account.deposit(500);
    account.withdraw(100);

    println!("{:?}", account);

    println!("{:?}", bank);
}
