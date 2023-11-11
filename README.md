Exante
======

HTTP client for Exante REST API version 3.0.

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
