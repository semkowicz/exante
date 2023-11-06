use serde::Deserialize;

/// Transaction.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Transaction ID.
    pub id: u64,
    /// Transaction UUID.
    pub uuid: String,
    /// Transaction type.
    pub operation_type: String,
    /// Timestamp of the transaction.
    pub timestamp: u64,
    /// Transaction amount.
    pub sum: String,
    /// Transaction asset.
    pub asset: String,
    /// Transaction account ID.
    pub account_id: String,
    /// Transaction financial instrument.
    pub symbol_id: Option<String>, // Differs: documentation states "required"
    /// Transaction order ID.
    pub order_id: Option<String>, // Differs: documentation states "required"
    /// Transaction position in the order.
    pub order_pos: Option<i64>, // Differs: documentation states "required"
    /// Value date.
    pub value_date: String,
    /// Parent transaction UUID.
    pub parent_uuid: Option<String>,
}
