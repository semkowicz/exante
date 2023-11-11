use exante::client::{AccountType, Client};

static EXANTE_API_KEY: &str = "";
static EXANTE_SECRET_KEY: &str = "";

fn create_client() -> Client {
    Client::new(AccountType::Demo, EXANTE_API_KEY, EXANTE_SECRET_KEY)
}

#[tokio::test]
async fn test_builder() {
    let client = create_client();

    let account_id = client
        .get_user_accounts()
        .send()
        .await
        .unwrap()
        .get(0)
        .unwrap()
        .account_id
        .to_owned();
    println!("account[0] = {account_id:?}");

    let transactions = client
        .get_transactions()
        .filter_account_id(account_id)
        .limit(5)
        .send()
        .await
        .unwrap();

    transactions.iter().for_each(|t| println!("{t:?}"));
}
