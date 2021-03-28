
use crate::*;

mod binance;
mod coinbase;
mod coincap;
use self::binance::Binance;
use self::coinbase::Coinbase;
use self::coincap::Coincap;



pub trait DataSourceApi<T: Config> {
    fn get_ticker(symbol: CurrencyId, source: DataSourceEnum, bytes: Vec<u8>) -> Result<TickerPayloadDetail, Error<T>>;
}

pub fn get_ticker<T: Config>(symbol: CurrencyId, source: DataSourceEnum, bytes: Vec<u8>) -> Result<TickerPayloadDetail, Error<T>> {
    match source {
        DataSourceEnum::BINANCE => Binance::get_ticker(symbol, source, bytes),
        DataSourceEnum::COINBASE => Coinbase::get_ticker(symbol, source, bytes),
        DataSourceEnum::COINCAP => Coincap::get_ticker(symbol, source, bytes),
    }
}