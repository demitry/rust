use std::any::Any;

fn main() {
    println!("Hello, world!");
    test_scalars();
    test_object_creation();
}
fn test_scalars(){
    let is_account_active: bool = true;  // Equivalent to bool in C# - Account status
    let first_char_last_name: char = 'D';  // Equivalent to char in C# - First character of last name
    let num_overdrawn_transactions: i8 = -3;  // Equivalent to sbyte in C# - Number of overdrawn transactions
    let savings_account_balance: i16 = 15000;  // Equivalent to short in C# - Savings account balance
    let checking_account_balance: i32 = 50000;  // Equivalent to int in C# - Checking account balance
    let investment_portfolio_balance: i64 = 1000000;  // Equivalent to long in C# - Investment portfolio balance
    let cryptocurrency_balance: i128 = 500000000000000000000000000;  // Int128 in C# - Cryptocurrency balance
    let selected_savings_plan_index: usize = 2;  // nint in C# - Selected savings plan index
    let num_shares_owned: u8 = 50;  // Equivalent to byte in C# - Number of shares owned
    let retirement_account_balance: u16 = 7500;  //75000 doesn't fit in u16. Equivalent to ushort in C# - Retirement account balance
    let college_savings_account_balance: u32 = 25000;  // Equivalent to uint in C# - College savings account balance
    let mortgage_balance: u64 = 200000;  // Equivalent to ulong in C# - Mortgage balance
    let cryptocurrency_wallet_balance: u128 = 10000000000000000000000000000;  // UInt128 in .NET, there is no in C# - Cryptocurrency wallet balance
    let selected_credit_card_index: usize = 1;  // nuint in C# - Selected credit card index
    let stock_price: f32 = 45.67;  // Equivalent to float in C# - Stock price
    let foreign_currency_balance: f64 = 1500.99;  // Equivalent to double in C# - Foreign currency balance
    let precious_metal_balance: f64 = 25000.50;  // Equivalent to double in C# - Precious metal balance
    let government_bonds_balance: f64 = 100000.75;  // Equivalent to double in C# - Government bonds balance
    let void_result: () = ();  // Equivalent to void in C# - Void result

    println!("Account Status: {}", is_account_active);
    println!("First Character of Last Name: {}", first_char_last_name);
    println!("Number of Overdrawn Transactions: {}", num_overdrawn_transactions);
    println!("Savings Account Balance: {}", savings_account_balance);
    println!("Checking Account Balance: {}", checking_account_balance);
    println!("Investment Portfolio Balance: {}", investment_portfolio_balance);
    println!("Cryptocurrency Balance: {}", cryptocurrency_balance);
    println!("Selected Savings Plan Index: {}", selected_savings_plan_index);
    println!("Number of Shares Owned: {}", num_shares_owned);
    println!("Retirement Account Balance: {}", retirement_account_balance);
    println!("College Savings Account Balance: {}", college_savings_account_balance);
    println!("Mortgage Balance: {}", mortgage_balance);
    println!("Cryptocurrency Wallet Balance: {}", cryptocurrency_wallet_balance);
    println!("Selected Credit Card Index: {}", selected_credit_card_index);
    println!("Stock Price: {}", stock_price);
    println!("Foreign Currency Balance: {}", foreign_currency_balance);
    println!("Precious Metal Balance: {}", precious_metal_balance);
    println!("Government Bonds Balance: {}", government_bonds_balance);
    println!("Void Result: {:?}", void_result);
}

struct AccountInfo {
    is_active: bool,
    account_number: String,
    balance: f64,
    owner_name: String,
}

fn test_object_creation(){
    // Create an instance of AccountInfo
    let account_info = AccountInfo {
        is_active: true,
        account_number: String::from("123456789"),
        balance: 10000.50,
        owner_name: String::from("John Doe"),
    };

    let customer_account_info: Option<Box<dyn Any>> = Some(Box::new(account_info));  // Equivalent to Object in C# - Customer account information

    println!("Customer Account Information: {:?}", customer_account_info);

    // Print out account_info with its values
    if let Some(account_info) = customer_account_info {
        if let Ok(account_info) = account_info.downcast::<crate::AccountInfo>() {
            println!("Account Information:");
            println!("Is Active: {}", account_info.is_active);
            println!("Account Number: {}", account_info.account_number);
            println!("Balance: {}", account_info.balance);
            println!("Owner Name: {}", account_info.owner_name);
        } else {
            println!("Failed to downcast account_info to AccountInfo type.");
        }
    } else {
        println!("customer_account_info is None.");
    }
}