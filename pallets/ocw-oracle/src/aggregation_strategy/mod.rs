
use crate::*;

mod median;
use self::median::*;

pub trait AggregationStrategyApi<T: Config> {
    fn aggregate_price(round_index: &RoundIndex<T::BlockNumber>, provider: &Vec<T::AccountId>, currency_id: &CurrencyId)-> Result<PriceDetail, Error<T>>;
}

pub fn aggregate_price<T: Config>(aggregate_strategy:AggregationStrategyEnum,round_index: &RoundIndex<T::BlockNumber>,provider: &Vec<T::AccountId>, currency_id: &CurrencyId)-> Result<PriceDetail, Error<T>> {
    match aggregate_strategy {
        AggregationStrategyEnum::EMERGENCY => Err(<Error<T>>::NotImplement.into()),
        AggregationStrategyEnum::MEDIAN => Median::aggregate_price(round_index, provider, currency_id),
        AggregationStrategyEnum::AVERAGE => Err(<Error<T>>::NotImplement.into()),
    }
}