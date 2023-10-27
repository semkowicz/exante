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
}

impl Client {
    /// Constructs a new client using provided account type and credentials.
    pub fn new(account_type: AccountType, _api_key: &str, _secret_key: &str) -> Self {
        let base = match account_type {
            AccountType::Demo => "https://api-demo.exante.eu",
            AccountType::Live => "https://api-live.exante.eu",
        };

        Self {
            http: HTTPClient::default(base),
        }
    }

    pub fn http(&self) -> &HTTPClient {
        &self.http
    }
}
