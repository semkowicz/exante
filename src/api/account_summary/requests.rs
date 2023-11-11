use crate::api::account_summary::responses::AccountSummary;
use derive_new::new;
use rustify_derive::Endpoint;

/// A request to get the summary for the specified account.
///
/// This implements the request structure for the "get account summary" endpoint.
#[derive(Debug, Endpoint, new)]
#[endpoint(
    path = "md/3.0/summary/{self.id}/{self.currency}",
    response = "AccountSummary"
)]
pub struct GetAccountSummary {
    /// Account ID to get summary.
    ///
    /// Example: "ABC1234.001".
    #[endpoint(skip)]
    pub id: String,
    /// Currency to convert summary.
    ///
    /// Example: "USD".
    #[endpoint(skip)]
    pub currency: String,
}

/// A request to get the summary for the specified account and session date.
///
/// This implements the request structure for the "get account summary by date" endpoint.
#[derive(Debug, Endpoint, new)]
#[endpoint(
    path = "md/3.0/summary/{self.id}/{self.date}/{self.currency}",
    response = "AccountSummary"
)]
pub struct GetAccountSummaryByDate {
    /// Account ID to get summary.
    ///
    /// Example: "ABC1234.001".
    #[endpoint(skip)]
    pub id: String,
    /// Session date of the account summary.
    ///
    /// Example: "2013-02-16".
    #[endpoint(skip)]
    pub date: String,
    /// Currency to convert summary.
    ///
    /// Example: "USD".
    #[endpoint(skip)]
    pub currency: String,
}
