use crate::api::account_summary::{GetAccountSummary, GetAccountSummaryByDate};
use crate::api::accounts::GetUserAccounts;
use crate::api::cross_rates::{GetAvailableCurrencies, GetCrossRate};
use crate::api::transactions::GetTransactions;
use crate::api::RequestBuilder;
use crate::middleware::Middle;
use rustify::clients::reqwest::Client as HTTPClient;
use rustify::errors::ClientError;
use rustify::Endpoint;

/// Exante account type.
pub enum AccountType {
    Demo,
    Live,
}

/// A Client for sending requests to Exante server.
/// Depending on account type, different server endpoint will be used for sending requests.
pub struct Client {
    http: HTTPClient,
    middle: Middle,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        // Rustify Client does not implement Clone.
        let http_client = HTTPClient {
            http: self.http.http.clone(),
            base: self.http.base.clone(),
        };

        Self {
            http: http_client,
            middle: self.middle.clone(),
        }
    }
}

impl Client {
    /// Constructs a new client using provided account type and credentials.
    pub fn new(account_type: AccountType, api_key: &str, secret_key: &str) -> Self {
        let base = match account_type {
            AccountType::Demo => "https://api-demo.exante.eu",
            AccountType::Live => "https://api-live.exante.eu",
        };

        Self {
            http: HTTPClient::default(base),
            middle: Middle::new(api_key, secret_key),
        }
    }

    /// Executes endpoint.
    pub async fn execute<E>(&self, endpoint: E) -> Result<<E as Endpoint>::Response, ClientError>
    where
        E: Endpoint,
    {
        endpoint
            .with_middleware(&self.middle)
            .exec(&self.http)
            .await?
            .parse()
    }

    /// Convenience method to make `GetUserAccounts` request.
    pub fn get_user_accounts(&self) -> RequestBuilder<GetUserAccounts> {
        let req = GetUserAccounts::new();
        RequestBuilder::new(self.clone(), req)
    }

    /// Convenience method to make `GetAvailableCurrencies` request.
    pub fn get_available_currencies(&self) -> RequestBuilder<GetAvailableCurrencies> {
        let req = GetAvailableCurrencies::new();
        RequestBuilder::new(self.clone(), req)
    }

    /// Convenience method to make `GetCrossRate` request.
    pub fn get_cross_rate(&self, from: String, to: String) -> RequestBuilder<GetCrossRate> {
        let req = GetCrossRate::new(from, to);
        RequestBuilder::new(self.clone(), req)
    }

    /// Convenience method to make `GetAccountSummary` request.
    pub fn get_account_summary(
        &self,
        account_id: String,
        currency: String,
    ) -> RequestBuilder<GetAccountSummary> {
        let req = GetAccountSummary::new(account_id, currency);
        RequestBuilder::new(self.clone(), req)
    }

    /// Convenience method to make `GetAccountSummaryByDate` request.
    pub fn get_account_summary_by_date(
        &self,
        account_id: String,
        currency: String,
        date: String,
    ) -> RequestBuilder<GetAccountSummaryByDate> {
        let req = GetAccountSummaryByDate::new(account_id, currency, date);
        RequestBuilder::new(self.clone(), req)
    }

    /// Convenience method to make `GetTransactions` request.
    pub fn get_transactions(&self) -> RequestBuilder<GetTransactions> {
        let req = GetTransactions::new();
        RequestBuilder::new(self.clone(), req)
    }
}
