use crate::api::request::RequestBuilder;
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
    /// Filter by transaction UUID.
    #[endpoint(query)]
    pub uuid: Option<String>,
    /// Filter by account ID.
    #[endpoint(query)]
    pub account_id: Option<String>,
    /// Filter by financial instrument.
    #[endpoint(query)]
    pub symbol_id: Option<String>,
    /// Filter by asset.
    #[endpoint(query)]
    pub asset: Option<String>,
    /// Transaction type or comma-separated list of transaction types to filter.
    #[endpoint(query)]
    pub operation_type: Option<String>,
    /// Offset to list transactions.
    #[endpoint(query)]
    pub offset: Option<i64>,
    /// Limit response to this amount of transactions.
    #[endpoint(query)]
    pub limit: Option<i64>,
    /// Order transactions by `Descending` or `Ascending`.
    #[endpoint(query)]
    pub order: Option<Order>,
    /// Filter transactions from date.
    #[endpoint(query)]
    pub from_date: Option<String>,
    /// Filter transactions to date.
    #[endpoint(query)]
    pub to_date: Option<String>,
    /// Filter by order ID.
    #[endpoint(query)]
    pub order_id: Option<String>,
    /// Filter by the position in the order.
    #[endpoint(query)]
    pub order_pos: Option<i64>,
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
}

impl RequestBuilder<GetTransactions> {
    /// Filters transactions by transaction UUID.
    pub fn filter_uuid(mut self, uuid: String) -> Self {
        self.request_mut().uuid = Some(uuid);
        self
    }

    /// Filters transactions by account ID.
    pub fn filter_account_id(mut self, account_id: String) -> Self {
        self.request_mut().account_id = Some(account_id);
        self
    }

    /// Filters transactions by financial instrument symbol.
    pub fn filter_symbol_id(mut self, symbol_id: String) -> Self {
        self.request_mut().symbol_id = Some(symbol_id);
        self
    }

    /// Filters transactions by asset.
    pub fn filter_asset(mut self, asset: String) -> Self {
        self.request_mut().asset = Some(asset);
        self
    }

    /// Filters transactions by operation type.
    pub fn filter_operation_type(mut self, operation_type: String) -> Self {
        self.request_mut().operation_type = Some(operation_type);
        self
    }

    /// Filters transactions from date.
    pub fn filter_from_date(mut self, date: String) -> Self {
        self.request_mut().from_date = Some(date);
        self
    }

    /// Filters transactions to date.
    pub fn filter_to_date(mut self, date: String) -> Self {
        self.request_mut().to_date = Some(date);
        self
    }

    /// Filters transactions by order ID.
    pub fn filter_order_id(mut self, order_id: String) -> Self {
        self.request_mut().order_id = Some(order_id);
        self
    }

    /// Filters transactions by position in order.
    pub fn filter_order_pos(mut self, order_pos: i64) -> Self {
        self.request_mut().order_pos = Some(order_pos);
        self
    }

    /// Offset the list by this number of transactions.
    pub fn offset(mut self, offset: i64) -> Self {
        self.request_mut().offset = Some(offset);
        self
    }

    /// Limit response to this number of transactions.
    pub fn limit(mut self, limit: i64) -> Self {
        self.request_mut().limit = Some(limit);
        self
    }

    /// Sets the order in which the transaction list should be sorted.
    pub fn order(mut self, order: Order) -> Self {
        self.request_mut().order = Some(order);
        self
    }
}
