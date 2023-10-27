use crate::middleware::Middle;
use rustify::clients::reqwest::Client as HTTPClient;

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

    pub fn http(&self) -> &HTTPClient {
        &self.http
    }

    pub fn middle(&self) -> &Middle {
        &self.middle
    }
}
