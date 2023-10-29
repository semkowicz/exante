use crate::api::cross_rates::responses::{CrossRate, Currencies};
use derive_new::new;
use rustify_derive::Endpoint;

/// A request to get the list of available currencies.
///
/// This implements the request structure for the "get list of available currencies" endpoint.
#[derive(Debug, Endpoint, new)]
#[endpoint(path = "md/3.0/crossrates", response = "Currencies")]
pub struct GetAvailableCurrencies {}

/// A request to get the cross rate from one currency to another.
///
/// This implements the request structure for the "get crossrate" endpoint.
#[derive(Debug, Endpoint, new)]
#[endpoint(
    path = "md/3.0/crossrates/{self.from}/{self.to}",
    response = "CrossRate"
)]
pub struct GetCrossRate {
    #[endpoint(skip)]
    from: String,
    #[endpoint(skip)]
    to: String,
}
