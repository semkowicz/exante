use exante::api::accounts::requests::GetUserAccounts;
use exante::api::cross_rates::requests::{GetAvailableCurrencies, GetCrossRate};
use exante::client::*;

static EXANTE_API_KEY: &str = "";
static EXANTE_SECRET_KEY: &str = "";

fn create_client() -> Client {
    Client::new(AccountType::Demo, EXANTE_API_KEY, EXANTE_SECRET_KEY)
}

#[tokio::test]
async fn get_user_accounts() {
    let endpoint = GetUserAccounts::new();
    let accounts = create_client().execute(endpoint).await.unwrap();
    println!("accounts = {:?}", accounts);
}

#[tokio::test]
async fn get_currencies() {
    let endpoint = GetAvailableCurrencies::new();
    let accounts = create_client().execute(endpoint).await.unwrap();
    println!("currencies = {:?}", accounts);
}

#[tokio::test]
async fn get_cross_rate() {
    let endpoint = GetCrossRate::new("USD".to_owned(), "PLN".to_owned());
    let cross_rate = create_client().execute(endpoint).await.unwrap();
    println!("{cross_rate:?}");
}
