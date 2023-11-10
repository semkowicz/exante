use crate::client::Client;
use rustify::errors::ClientError;
use rustify::Endpoint;

/// A builder to construct a request.
pub struct RequestBuilder<R> {
    client: Client,
    request: R,
}

impl<R: Endpoint> RequestBuilder<R> {
    /// Constructs a new `RequestBuilder` from provided `Client` and request.
    pub fn new(client: Client, request: R) -> RequestBuilder<R> {
        Self { client, request }
    }

    /// Builds a request, which can be inspected, modified and executed with `Client::execute()`.
    pub fn build(self) -> R {
        self.request
    }

    /// Constructs the request and sends it to the target server. Returns a response.
    pub async fn send(self) -> Result<<R>::Response, ClientError> {
        self.client.execute(self.request).await
    }

    pub(in crate::api) fn request_mut(&mut self) -> &mut R {
        &mut self.request
    }
}
