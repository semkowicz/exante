use crate::api::cross_rates::responses::Currencies;
use derive_new::new;
use rustify_derive::Endpoint;

/// A request to get the list of available currencies.
///
/// This implements the request structure for the "get list of available currencies" endpoint.
#[derive(Debug, Endpoint, new)]
#[endpoint(path = "md/3.0/crossrates", response = "Currencies")]
pub struct GetAvailableCurrencies {}
