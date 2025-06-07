//! Example of implementation of requests OKX API V5.

use crate::entities::InstrumentType;
use crate::requests::GetInstruments;
use anyhow::Result;
use pretty_rest::reqwest::client::RestClient;
use pretty_rest::reqwest::header::RestHeaderBuilder;
use reqwest::{ClientBuilder, Url};
use std::time::Duration;

mod entities {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum InstrumentType {
        Spot,
        Margin,
        Swap,
        Futures,
        Option,
        Any,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Instrument {
        #[serde(rename = "instType")]
        pub inst_type: InstrumentType,
        #[serde(rename = "instId")]
        pub inst_id: String,
    }
}
mod requests {
    use crate::entities::{Instrument, InstrumentType};
    use crate::response::RestResponse;
    use pretty_rest::{Request, RequestMethod};
    use serde::Serialize;

    /// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-instruments
    /// ## Get instruments
    /// Retrieve a list of instruments with open contracts.
    ///
    /// Rate Limit: 20 requests per 2 seconds
    /// Rate limit rule: IP + instrumentType
    ///
    /// ## HTTP Request
    /// GET /api/v5/public/instruments
    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetInstruments<'a> {
        /// Instrument type
        /// SPOT
        /// MARGIN
        /// SWAP
        /// FUTURES
        /// OPTION
        pub inst_type: InstrumentType,
        /// Underlying
        /// Only applicable to FUTURES/SWAP/OPTION.If instType is OPTION, either uly or instFamily is required.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uly: Option<&'a str>,
        /// Instrument family
        /// Only applicable to FUTURES/SWAP/OPTION. If instType is OPTION, either uly or instFamily is required.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_family: Option<&'a str>,
        /// Instrument ID
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_id: Option<&'a str>,
    }

    impl Request for GetInstruments<'_> {
        const METHOD: RequestMethod = RequestMethod::Get;
        const PATH: &'static str = "public/instruments";
        const IN_URI: bool = true;
        type Response = RestResponse<Vec<Instrument>>;
    }
}
mod response {
    use anyhow::{Result, bail};
    use pretty_rest::Response;
    use serde::Deserialize;
    use serde::de::DeserializeOwned;
    use serde_aux::prelude::deserialize_number_from_string;
    use std::fmt::Debug;

    #[derive(Debug, Deserialize)]
    pub struct RestResponse<T> {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub code: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub msg: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<T>,
    }

    impl<T: DeserializeOwned + Debug> Response for RestResponse<T> {
        type Data = T;

        fn extract(self) -> Result<T> {
            const SUCCESS_CODE: u64 = 0;
            match self.code {
                SUCCESS_CODE => self.data.ok_or_else(|| anyhow::anyhow!("no data")),
                err_code => bail!(
                    self.msg
                        .unwrap_or_else(|| format!("error code: {err_code}"))
                ),
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_instruments() -> Result<()> {
    // Arrange
    let tcp_client = ClientBuilder::new()
        .tcp_nodelay(true)
        .tcp_keepalive(Duration::from_secs(30))
        .timeout(Duration::from_secs(30))
        .build()?;

    let rest_client = RestClient::new(Url::parse("https://www.okx.com/api/v5/")?, tcp_client)
        .header_builder(|builder: &mut RestHeaderBuilder| {
            builder.content_type("application/json");
        });

    // Act / Assert
    let _response = rest_client
        .call(GetInstruments {
            inst_type: InstrumentType::Spot,
            uly: None,
            inst_family: None,
            inst_id: None,
        })
        .await?;

    Ok(())
}
