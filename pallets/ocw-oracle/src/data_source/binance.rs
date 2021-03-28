use crate::*;
pub struct Binance;

impl<T: Config> DataSourceApi<T> for Binance {
    fn get_ticker(symbol: CurrencyId, data_source_enum: DataSourceEnum, bytes: Vec<u8>) -> Result<TickerPayloadDetail, Error<T>> {
        let resp_str =
            str::from_utf8(&bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;
        let json: Ticker =
            serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;
        let price = Pallet::<T>::to_price(json.price)?;
        let now = T::Time::now();
        let timestamp: Timestamp = now.try_into().or(Err(Error::<T>::ParseTimestampError))?;
        let r = TickerPayloadDetail {
            symbol,
            data_source_enum,
            price,
            timestamp,
        };

        Ok(r)
    }
}

/// {
///     "symbol": "DOTUSDT",
///     "price": "32.02420000"
/// }
#[derive(Deserialize, Encode, Decode, Default, Clone)]
struct Ticker {
    #[serde(deserialize_with = "de_string_to_bytes")]
    symbol: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    price: Vec<u8>,
}

