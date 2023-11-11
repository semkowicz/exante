use crate::ClientError;
use http::HeaderValue;
use rustify::{Endpoint, MiddleWare};

#[derive(Clone)]
pub struct Middle {
    credentials: HeaderValue,
}

impl Middle {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            credentials: basic_auth(api_key, secret_key),
        }
    }
}

impl MiddleWare for Middle {
    fn request<E: Endpoint>(
        &self,
        _endpoint: &E,
        req: &mut http::Request<Vec<u8>>,
    ) -> Result<(), ClientError> {
        req.headers_mut()
            .append(http::header::AUTHORIZATION, self.credentials.clone());
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

fn basic_auth(username: &str, password: &str) -> HeaderValue {
    use base64::prelude::BASE64_STANDARD;
    use base64::write::EncoderWriter;
    use std::io::Write;

    let mut buf = b"Basic ".to_vec();
    {
        let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
        let _ = write!(encoder, "{username}:{password}");
    }
    let mut header = HeaderValue::from_bytes(&buf).expect("base64 is always valid HeaderValue");
    header.set_sensitive(true);
    header
}
