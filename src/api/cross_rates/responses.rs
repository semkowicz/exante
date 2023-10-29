use serde::Deserialize;

/// Response structure for the "get list of available currencies" method.
#[derive(Debug, Deserialize)]
pub struct Currencies {
    /// List of available currencies.
    pub currencies: Vec<String>,
}

/// Cross rate from one currency to another.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossRate {
    /// Cross rate pair.
    pub pair: String,
    /// Optional symbol id, which can be used to request history or subscribe to feed.
    pub symbol_id: Option<String>,
    /// Current cross rate.
    pub rate: String,
}
