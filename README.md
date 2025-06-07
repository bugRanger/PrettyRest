# Название проекта
PrettyREST — это дизайн для REST-запросов, описываемый всего двумя чертами, которые значительно упрощают ваш код.

## Example
```rust
/// Request parameters Order book.
#[derive(Debug, Deserialize)]
struct GetOrderBookSnapshot {
    symbol_id: String,
}

impl Request for GetOrderBookSnapshot {
    const METHOD: RequestMethod = RequestMethod::Get;
    const PATH: &'static str = "public/order_book";
    const IN_URI: bool = true;
    type Response = Message<OrderBookSnapshot>;
}

#[derive(Debug, Deserialize)]
pub struct OrderBookSnapshot {
    pub bids: Vec<f64>,
    pub asks: Vec<f64>,
}

/// A message containing information about the results of the query execution and the requested data if successful.
#[derive(Debug, Deserialize)]
pub struct ResponseMessage<T> {
    pub code: u64,
    pub error: String,
    pub data: T
}

impl<T: DeserializeOwned + Debug> Response for ResponseMessage<T> {
    type Data = T;

    fn extract(self) -> Result<T> {
        const SUCCESS_CODE: u64 = 0;
        match self.code {
            SUCCESS_CODE => self.data.ok_or_else(|| anyhow::anyhow!("no data")),
            err_code => bail!(
                    self.error.unwrap_or_else(|| format!("Unknown error code: {err_code}"))
                ),
        }
    }
}
```

## To do
- [x] Добавить [reqwest](https://docs.rs/reqwest/latest/reqwest/)
- [ ] Добавить [hyper](https://docs.rs/hyper/latest/hyper/)\

## License

[LICENSE-MIT](https://github.com/bugRanger/PrettyRest/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>