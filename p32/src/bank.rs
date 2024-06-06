use std::fmt;
use std::collections::HashMap;
// Create User struct with String field name, u64 field credit_line, and i64 field balance (positive number means debit, negative credit)
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64,
}

impl User {
    // Constructor function for User
    pub fn new(name: String, credit_line: u64, balance: i64) -> User {
        User {
            name,
            credit_line,
            balance,
        }
    }

    // Other methods for User can be added here
}

//Create Bank struct which contains list of Users, name of the bank, u64 fields credit_interest and debit_interest in basis points, i.e. 0.01%
#[derive(Debug, Clone)]
pub struct Bank {
    pub users: Vec<User>,
    pub name: String,
    pub credit_interest: u64, // Basis points for credit interest (e.g., 1000 for 10%)
    pub debit_interest: u64,  // Basis points for debit interest (e.g., 500 for 5%)
}

impl Bank {
    // Constructor function for Bank
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Bank {
        Bank {
            users: Vec::new(),
            name,
            credit_interest,
            debit_interest,
        }
    }

    // Other methods for Bank can be added here
    pub fn calc_balance(&self) -> (i64, u64) {
        let mut liabilities = 0;
        let mut assets = 0;

        for user in &self.users {
            if user.balance < 0 {
                liabilities += user.balance;
            } else {
                assets += user.balance as u64;
            }
        }

        (liabilities, assets)
    }

    pub fn transfer_funds(&mut self, from_user: &str, to_user: &str, amount: u64) -> Result<(), &'static str> {
        let mut from_index = None;
        let mut to_index = None;

        for (index, user) in self.users.iter_mut().enumerate() {
            if user.name == from_user {
                from_index = Some(index);
            }
            if user.name == to_user {
                to_index = Some(index);
            }
        }

        if let Some(from_idx) = from_index {
            if let Some(to_idx) = to_index {
                let from_balance = self.users[from_idx].balance;
                if from_balance >= amount as i64 {
                    self.users[from_idx].balance -= amount as i64;
                    self.users[to_idx].balance += amount as i64;
                    Ok(())
                } else {
                    Err("Insufficient funds")
                }
            } else {
                Err("Recipient user not found")
            }
        } else {
            Err("Sender user not found")
        }
    }

    pub fn accrue_interest(&mut self) {
        for user in &mut self.users {
            if user.balance > 0 {
                let interest_amount = user.balance as f64 * (self.credit_interest as f64 / 10000.0);
                user.balance += interest_amount.round() as i64;
            } else if user.balance < 0 {
                let interest_amount = user.balance.abs() as f64 * (self.debit_interest as f64 / 10000.0);
                user.balance -= interest_amount.round() as i64;
            }
        }
    }

    pub fn merge_bank(&mut self, mut other: Bank) {
        let mut user_map: HashMap<String, (u64, i64)> = HashMap::new();

        for user in self.users.drain(..) {
            user_map.insert(user.name.clone(), (user.credit_line, user.balance));
        }

        for user in other.users.drain(..) {
            if let Some((credit_line, balance)) = user_map.get_mut(&user.name) {
                *credit_line = (*credit_line).max(user.credit_line);
                *balance += user.balance;
            } else {
                user_map.insert(user.name.clone(), (user.credit_line, user.balance));
            }
        }

        self.users = user_map.into_iter().map(|(name, (credit_line, balance))| User {
            name,
            credit_line,
            balance,
        }).collect();
    }
}

pub trait UserBank {
    type UserType;
    type CreditLineType;
    type BalanceType;

    fn get_user(&self) -> &Self::UserType;
    fn get_credit_line(&self) -> &Self::CreditLineType;
    fn get_balance(&self) -> &Self::BalanceType;
}

impl UserBank for User {
    type UserType = String;
    type CreditLineType = u64;
    type BalanceType = i64;

    fn get_user(&self) -> &Self::UserType {
        &self.name
    }

    fn get_credit_line(&self) -> &Self::CreditLineType {
        &self.credit_line
    }

    fn get_balance(&self) -> &Self::BalanceType {
        &self.balance
    }
}

impl UserBank for Bank {
    type UserType = Vec<User>;
    type CreditLineType = String;
    type BalanceType = (i64, u64);

    fn get_user(&self) -> &Self::UserType {
        &self.users
    }
    // fn get_user(&self) -> Vec<User> {
    //     self.users
    // }

    fn get_credit_line(&self) -> &Self::CreditLineType {
        &self.name
    }

    fn get_balance(&self) -> &Self::BalanceType {
        &self.calc_balance()
    }
}


impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Bank Name: {}", self.name)?;
        writeln!(f, "Credit Interest: {} basis points", self.credit_interest)?;
        writeln!(f, "Debit Interest: {} basis points", self.debit_interest)?;
        writeln!(f, "Users:")?;
        for user in &self.users {
            writeln!(f, "{}: {} (Credit Line: {}, Balance: {})", user.name, if user.balance >= 0 { "Asset" } else { "Liability" }, user.credit_line, user.balance)?;
        }
        Ok(())
    }
}
