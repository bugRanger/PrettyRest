pub mod reqwest;

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// The Request Method.
///
/// This type also contains constants for a number of common HTTP methods such as GET, POST, etc.
#[derive(Copy, Clone)]
pub enum RequestMethod {
    Get,
    Post,
    Delete,
    Put,
}

impl RequestMethod {
    pub fn as_str(&self) -> &str {
        match self {
            RequestMethod::Get => "GET",
            RequestMethod::Post => "POST",
            RequestMethod::Delete => "DELETE",
            RequestMethod::Put => "PUT",
        }
    }
}

pub trait Request: Serialize {
    const METHOD: RequestMethod;
    const PATH: &'static str;
    const IN_URI: bool = false;
    type Response: Response;
}

pub trait Response: DeserializeOwned + Debug {
    type Data: DeserializeOwned + Debug;

    fn extract(self) -> anyhow::Result<Self::Data>;
}
