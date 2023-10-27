use rustify::errors::ClientError;
use rustify::{Endpoint, MiddleWare};

pub struct Middle {
    _api_key: String,
    _secret_key: String,
}

impl Middle {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            _api_key: api_key.to_string(),
            _secret_key: secret_key.to_string(),
        }
    }
}

impl MiddleWare for Middle {
    fn request<E: Endpoint>(
        &self,
        _endpoint: &E,
        _req: &mut http::Request<Vec<u8>>,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    fn response<E: Endpoint>(
        &self,
        _endpoint: &E,
        _resp: &mut http::Response<Vec<u8>>,
    ) -> Result<(), ClientError> {
        Ok(())
    }
}
