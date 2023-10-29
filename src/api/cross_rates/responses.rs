use serde::Deserialize;

/// Response structure for the "get list of available currencies" method.
#[derive(Debug, Deserialize)]
pub struct Currencies {
    /// List of available currencies.
    pub currencies: Vec<String>,
}
