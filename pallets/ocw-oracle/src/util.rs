use crate::*;


impl<T: Config> Pallet<T> {
    pub(crate) fn to_price(val_u8: Vec<u8>) -> Result<Price, Error<T>> {
        let val_f64: f64 = core::str::from_utf8(&val_u8)
            .map_err(|_| {
                debug::error!("val_u8 convert to string error");
                <Error<T>>::ConvertToStringError
            })?
            .parse::<f64>()
            .map_err(|_| {
                debug::error!("string convert to f64 error");
                <Error<T>>::ParsingToF64Error
            })?;

        let price = (val_f64 * 10f64.powi(T::PricePrecision::get() as i32)).round() as Price;
        Ok(price)
    }
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}

