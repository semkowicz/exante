use crate::api::transactions::responses::Transaction;
use rustify_derive::Endpoint;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Order {
    #[serde(rename = "ASC")]
    Ascending,
    #[serde(rename = "DESC")]
    Descending,
}

/// A request to get the list of transactions with the specified filter.
///
/// This implements the request structure for the "get transactions" endpoint.
#[derive(Debug, Endpoint, Serialize)]
#[endpoint(path = "md/3.0/transactions", response = "Vec<Transaction>")]
#[serde(rename_all = "camelCase")]
pub struct GetTransactions {
    #[endpoint(query)]
    uuid: Option<String>,
    #[endpoint(query)]
    account_id: Option<String>,
    #[endpoint(query)]
    symbol_id: Option<String>,
    #[endpoint(query)]
    asset: Option<String>,
    #[endpoint(query)]
    operation_type: Option<String>,
    #[endpoint(query)]
    offset: Option<i64>,
    #[endpoint(query)]
    limit: Option<i64>,
    #[endpoint(query)]
    order: Option<Order>,
    #[endpoint(query)]
    from_date: Option<String>,
    #[endpoint(query)]
    to_date: Option<String>,
    #[endpoint(query)]
    order_id: Option<String>,
    #[endpoint(query)]
    order_pos: Option<i64>,
}

impl GetTransactions {
    /// Constructs a new `GetTransactions` request.
    pub fn new() -> Self {
        Self {
            uuid: None,
            account_id: None,
            symbol_id: None,
            asset: None,
            operation_type: None,
            offset: None,
            limit: None,
            order: None,
            from_date: None,
            to_date: None,
            order_id: None,
            order_pos: None,
        }
    }

    /// Filters transactions by transaction UUID.
    pub fn filter_uuid(&mut self, uuid: String) {
        self.uuid = Some(uuid);
    }

    /// Filters transactions by account ID.
    pub fn filter_account_id(&mut self, account_id: String) {
        self.account_id = Some(account_id);
    }

    /// Filters transactions by financial instrument symbol.
    pub fn filter_symbol_id(&mut self, symbol_id: String) {
        self.symbol_id = Some(symbol_id);
    }

    /// Filters transactions by asset.
    pub fn filter_asset(&mut self, asset: String) {
        self.asset = Some(asset);
    }

    /// Filters transactions by operation type.
    pub fn filter_operation_type(&mut self, operation_type: String) {
        self.operation_type = Some(operation_type);
    }

    /// Filters transactions from date.
    pub fn filter_from_date(&mut self, date: String) {
        self.from_date = Some(date);
    }

    /// Filters transactions to date.
    pub fn filter_to_date(&mut self, date: String) {
        self.to_date = Some(date);
    }

    /// Filters transactions by order ID.
    pub fn filter_order_id(&mut self, order_id: String) {
        self.order_id = Some(order_id);
    }

    /// Filters transactions by position in order.
    pub fn filter_order_pos(&mut self, order_pos: i64) {
        self.order_pos = Some(order_pos);
    }

    /// Offset the list by this number of transactions.
    pub fn offset(&mut self, offset: i64) {
        self.offset = Some(offset);
    }

    /// Limit response to this number of transactions.
    pub fn limit(&mut self, limit: i64) {
        self.limit = Some(limit);
    }

    /// Sets the order in which the transaction list should be sorted.
    pub fn order(&mut self, order: Order) {
        self.order = Some(order);
    }
}
