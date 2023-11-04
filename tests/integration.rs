use exante::api::account_summary::requests::{GetAccountSummary, GetAccountSummaryByDate};
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

#[tokio::test]
async fn get_account_summary() {
    let client = create_client();
    let accounts = client.execute(GetUserAccounts::new()).await.unwrap();
    let account_id = accounts.get(0).unwrap().account_id.to_owned();

    let endpoint = GetAccountSummary::new(account_id.clone(), "USD".to_owned());
    let summary = client.execute(endpoint).await.unwrap();
    println!("{summary:?}");

    let endpoint =
        GetAccountSummaryByDate::new(account_id, "2023-01-01".to_owned(), "USD".to_owned());
    let summary = client.execute(endpoint).await.unwrap();
    println!("{summary:?}");
}
