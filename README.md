Exante
======

HTTP client for Exante REST API version 3.0.

Example
-------

This asynchronous example uses [Tokio](https://tokio.rs), so your `Cargo.toml`
could look like this:

```toml
[dependencies]
exante = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

and then the code:

```rust
use exante::{AccountType, Client, ClientError};

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let accounts = Client::new(AccountType::Demo, "api-key", "secret-key")
        .get_user_accounts()
        .send()
        .await?;
    println!("{accounts:?}");
    Ok(())
}
```

Strings `api-key` and `secret-key` should be replaced with your own
authentication key.

More examples can be found in [examples](examples) directory.

Authentication
--------------

Basic HTTP authentication scheme is used for sending requests to Exante server.

In order to generate an authentication key, please follow the steps below.
1. Log in to your [Exante](https://exante.eu/) account.
2. Go to 'Settings' -> 'API Management'.
3. Press 'Add API Key' button in upper right corner.

Supported endpoints
-------------------

Client was implemented based on [Exante REST API documentation](https://api-live.exante.eu/api-docs/).
Version 3.0 is used for all requests.

The table below shows the currently implemented endpoints.

| API              | Supported |
|:-----------------|:---------:|
| Historical       |     -     |
| Accounts         |    Yes    |
| Daily change     |     -     |
| Cross rates      |    Yes    |
| Symbols          |     -     |
| Live feed        |     -     |
| Account summary  |    Yes    |
| Transactions     |    Yes    |
| Orders           |     -     |
| Orders stream    |     -     |
| Trades stream    |     -     |
