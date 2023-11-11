extern crate tokio;

use exante::{AccountType, Client};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
    let client = Client::new(AccountType::Demo, "api-key", "secret-key");

    // Get first user account.
    let account_id = client
        .get_user_accounts()
        .send()
        .await
        .map_err(|e| format!("Failed to get user accounts: {e}"))?
        .first()
        .ok_or_else(|| String::from("No account found"))?
        .account_id
        .to_owned();

    // Get 10 transactions for the first user account.
    let transactions = client
        .get_transactions()
        .filter_account_id(account_id)
        .limit(10)
        .send()
        .await
        .map_err(|e| format!("Failed to get transactions: {e}"))?;

    // Print transactions.
    transactions.iter().for_each(|t| println!("{t:?}"));

    Ok(())
}
