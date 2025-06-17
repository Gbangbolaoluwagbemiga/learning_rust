#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> u32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }
    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
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

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Oluwagbemiga"));
    let mut account2 = Account::new(1, String::from("Xcel"));

    account.summary();
    account.deposit(500);
    account2.deposit(300);
    account.withdraw(100);

    println!("{:?}", account);
    bank.add_account(account);
    bank.add_account(account2);
    let bank_summary = bank.summary();
    print!("your bank summary is {:#?}", bank_summary);
    println!("Total balance {}", bank.total_balance());

    match bank.accounts.get(0..2) {
        Some(value) => {
            println!("item:{:#?}", value);
        }
        None => {
            println!("Nothing at this index")
        }
    }

    // println!("{:#?}", bank.accounts.get(100));
}
