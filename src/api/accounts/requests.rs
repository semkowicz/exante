use crate::api::accounts::responses::Account;
use derive_new::new;
use rustify_derive::Endpoint;

/// A request to get the list of user accounts and their statuses.
///
/// This implements the request structure for the "get user accounts" endpoint.
#[derive(Debug, Endpoint, new)]
#[endpoint(path = "md/3.0/accounts", response = "Vec<Account>")]
pub struct GetUserAccounts {}
