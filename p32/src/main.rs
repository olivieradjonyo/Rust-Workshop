fn main() {
    // // Example usage
    // let user1 = p32::bank::User {
    //     name: String::from("Alice"),
    //     credit_line: 1000,
    //     balance: -500,
    // };

    // let user2 = p32::bank::User {
    //     name: String::from("Bob"),
    //     credit_line: 2000,
    //     balance: 100,
    // };

    // // Accessing fields
    // println!("User 1: {} (Credit Line: {}, Balance: {})", user1.name, user1.credit_line, user1.balance);
    // println!("User 2: {} (Credit Line: {}, Balance: {})", user2.name, user2.credit_line, user2.balance);
    // //-----------------------
    // // Example usage
    // let mut bank = p32::bank::Bank::new(String::from("My Bank"), 1000, 500);

    // // Adding users
    // bank.users.push(p32::bank::User {
    //     name: String::from("Alice"),
    //     credit_line: 1000,
    //     balance: -500,
    // });

    // bank.users.push(p32::bank::User {
    //     name: String::from("Bob"),
    //     credit_line: 2000,
    //     balance: 100,
    // });

    // // Accessing fields
    // println!("Bank Name: {}", bank.name);
    // println!("Credit Interest: {} basis points", bank.credit_interest);
    // println!("Debit Interest: {} basis points", bank.debit_interest);

    // // Iterating over users
    // for user in &bank.users {
    //     println!("User: {} (Credit Line: {}, Balance: {})", user.name, user.credit_line, user.balance);
    // }

    // //test for bank methods
    // println!("Initial bank state:");
    // println!("{}", bank);

    // let (liabilities, assets) = bank.calc_balance();
    // println!("Total liabilities: {}", liabilities);
    // println!("Total assets: {}", assets);

    // bank.transfer_funds("Bob", "Alice", 200).unwrap();
    // println!("After transferring funds:");
    // println!("{}", bank);

    // bank.accrue_interest();
    // println!("After accruing interest:");
    // println!("{}", bank);

    let mut bank1 = p32::bank::Bank::new(String::from("Bank 1"), 1000, 500);
    let mut bank2 = p32::bank::Bank::new(String::from("Bank 2"), 1000, 500);

    bank1.users.push(p32::bank::User {
        name: String::from("Alice"),
        credit_line: 1000,
        balance: -500,
    });

    bank1.users.push(p32::bank::User {
        name: String::from("Bob"),
        credit_line: 2000,
        balance: 100,
    });

    bank2.users.push(p32::bank::User {
        name: String::from("Charlie"),
        credit_line: 1500,
        balance: 200,
    });

    bank2.users.push(p32::bank::User {
        name: String::from("Bob"),
        credit_line: 2500,
        balance: -50,
    });

    println!("Before merge:");
    println!("{:?}", bank1);
    println!("{:?}", bank2);

    bank1.merge_bank(bank2);

    println!("After merge:");
    println!("{:?}", bank1);
    // Bank2 is effectively destroyed after merge
}
