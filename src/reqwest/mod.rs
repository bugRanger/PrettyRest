use crate::RequestMethod;
use reqwest::Method;

pub mod client;
pub mod header;

impl From<RequestMethod> for Method {
    fn from(val: RequestMethod) -> Self {
        match val {
            RequestMethod::Get => Method::GET,
            RequestMethod::Post => Method::POST,
            RequestMethod::Delete => Method::DELETE,
            RequestMethod::Put => Method::PUT,
        }
    }
}
