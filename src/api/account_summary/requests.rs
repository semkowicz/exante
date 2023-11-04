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
    #[endpoint(skip)]
    id: String,
    #[endpoint(skip)]
    currency: String,
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
    #[endpoint(skip)]
    id: String,
    #[endpoint(skip)]
    date: String,
    #[endpoint(skip)]
    currency: String,
}
