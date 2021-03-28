use crate::*;
pub struct Median;

impl<T: Config> AggregationStrategyApi<T> for Median {
    fn aggregate_price(round_index: &RoundIndex<T::BlockNumber>,provider: &Vec<T::AccountId>, currency_id: &CurrencyId)-> Result<PriceDetail, Error<T>>{
        let mut prices = vec![];
        provider.iter().for_each(|account_id|{
            Pallet::<T>::ocw_oracle_data_source().iter().for_each(|data_source_enum: &DataSourceEnum|{
                let ovp: Option<VecDeque<PriceDetailOf<T::BlockNumber>>> = Pallet::<T>::ocw_oracle_price(account_id, (data_source_enum,currency_id));
                if let Some(vp) = ovp {
                    if let Some(p) = vp.back(){
                        if round_index == &p.index {
                            prices.push(p.price);
                        }else {
                            debug::warn!("price round index is {:?}, while this round is {:?}", p.index, round_index);
                        }
                    }
                }
            });
        });
        let count = prices.len() as u32;
        prices.sort_by(|a, b| a.cmp(b));
        let median_index = count / 2;
        let now = T::Time::now();
        let timestamp: Timestamp = now.try_into().or(Err(Error::<T>::ParseTimestampError))?;
		Ok((prices[median_index as usize].clone(),timestamp))
    }
}
