use exante::api::accounts::requests::GetUserAccounts;
use exante::client::*;

static EXANTE_API_KEY: &str = "";
static EXANTE_SECRET_KEY: &str = "";

#[tokio::test]
async fn get_user_accounts() {
    let client = Client::new(AccountType::Demo, EXANTE_API_KEY, EXANTE_SECRET_KEY);
    let endpoint = GetUserAccounts::new();
    let accounts = client.execute(endpoint).await.unwrap();
    println!("accounts = {:?}", accounts);
}
