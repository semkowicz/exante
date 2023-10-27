use serde::Deserialize;

/// Account status.
#[derive(Debug, Deserialize)]
pub enum Status {
    ReadOnly,
    CloseOnly,
    Full,
}

/// User account and it's status.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// Account status.
    pub status: Status,
    /// Account ID.
    pub account_id: String,
}
