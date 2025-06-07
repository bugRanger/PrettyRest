use crate::RequestMethod;
use reqwest::Url;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub struct RestHeaderBuilder<'a> {
    pub method: RequestMethod,
    pub url: &'a Url,
    pub body: &'a str,
    headers: HeaderMap,
}

impl<'a> RestHeaderBuilder<'a> {
    pub fn new(method: RequestMethod, url: &'a Url, body: &'a str) -> Self {
        Self {
            method,
            url,
            body,
            headers: Default::default(),
        }
    }

    pub fn content_type(&mut self, value: impl Into<&'static str>) {
        self.headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static(value.into()),
        );
    }

    pub fn with(&mut self, key: impl Into<&'static str>, value: impl Into<&'static str>) {
        self.headers.insert(
            HeaderName::from_static(key.into()),
            HeaderValue::from_static(value.into()),
        );
    }

    pub(crate) fn build(self) -> HeaderMap {
        self.headers
    }
}
