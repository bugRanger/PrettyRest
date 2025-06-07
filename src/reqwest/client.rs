use crate::reqwest::header::RestHeaderBuilder;
use crate::{Request, Response};
use anyhow::{Result, bail};
use reqwest::{Client, Url};

#[derive(Clone)]
pub struct RestClient {
    host: Url,
    client: Client,
    header_build_fn: Option<fn(&mut RestHeaderBuilder)>,
}

impl RestClient {
    pub fn new(host: Url, client: Client) -> Self {
        Self {
            host,
            client,
            header_build_fn: None,
        }
    }

    pub fn header_builder(&mut self, headers_build_fn: fn(&mut RestHeaderBuilder)) {
        self.header_build_fn = Some(headers_build_fn);
    }

    pub async fn call<R: Request<Response = T>, T: Response>(&self, request: R) -> Result<T::Data> {
        let (params, body) = match R::IN_URI {
            true => (Some(serde_qs::to_string(&request)?), String::new()),
            false => (None, serde_json::to_string(&request)?),
        };

        let path = R::PATH;
        let host = match params {
            Some(params) => {
                let mut new_path = path.to_string();
                new_path.push('?');
                new_path.push_str(&params);
                self.host.join(&new_path)?
            }
            None => self.host.join(path)?,
        };

        let mut headers_builder = RestHeaderBuilder::new(R::METHOD, &host, &body);
        if let Some(headers_builder_fn) = self.header_build_fn {
            headers_builder_fn(&mut headers_builder);
        }

        let sent = self
            .client
            .request(R::METHOD.into(), host.clone())
            .headers(headers_builder.build())
            .body(body)
            .send()
            .await?;

        if let Err(err) = sent.error_for_status_ref() {
            bail!(err.to_string());
        }

        let body = sent.bytes().await?;
        let response = serde_json::from_slice::<R::Response>(&body)?;

        response.extract()
    }
}
