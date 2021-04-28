// Copyright 2021 Parallel Finance Developer.
// This file is part of Parallel Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::{pallet_prelude::*, traits::Time, transactional};
use frame_system::pallet_prelude::*;
use orml_traits::{DataFeeder, DataProvider, DataProviderExtended};
use primitives::*;
use sp_runtime::FixedPointNumber;

pub use module::*;

pub const CURRENCY_DECIMAL: u32 = 18;

pub type TimeStampedPrice = orml_oracle::TimestampedValue<OraclePrice, Moment>;

#[frame_support::pallet]
pub mod module {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The data source, such as Oracle.
        type Source: DataProvider<CurrencyId, OraclePrice>
            + DataFeeder<CurrencyId, OraclePrice, Self::AccountId>
            + DataProviderExtended<CurrencyId, TimeStampedPrice>;

        /// The stable currency id, it should be USDT in Parallel.
        #[pallet::constant]
        type GetStableCurrencyId: Get<CurrencyId>;

        /// The fixed prices of stable currency, it should be 1 USD in Parallel.
        #[pallet::constant]
        type StableCurrencyFixedPrice: Get<OraclePrice>;

        /// Time provider
        type Time: Time;

        /// The origin which may set prices feed to system.
        type FeederOrigin: EnsureOrigin<Self::Origin>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Set emergency price. \[currency_id, price_detail\]
        SetEmergencyPrice(CurrencyId, OraclePrice),
        /// Reset emergency price. \[currency_id\]
        ResetEmergencyPrice(CurrencyId),
    }

    /// Mapping from currency id to it's emergency price
    #[pallet::storage]
    #[pallet::getter(fn emergency_price)]
    pub type EmergencyPrice<T: Config> =
        StorageMap<_, Twox64Concat, CurrencyId, OraclePrice, OptionQuery>;

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set emergency price
        #[pallet::weight(100)]
        #[transactional]
        pub fn set_price(
            origin: OriginFor<T>,
            currency_id: CurrencyId,
            price: OraclePrice,
        ) -> DispatchResultWithPostInfo {
            T::FeederOrigin::ensure_origin(origin)?;
            <Pallet<T> as EmergencyPriceFeeder<CurrencyId, OraclePrice>>::set_emergency_price(
                currency_id,
                price,
            );
            Ok(().into())
        }

        /// Reset emergency price
        #[pallet::weight(100)]
        #[transactional]
        pub fn reset_price(
            origin: OriginFor<T>,
            currency_id: CurrencyId,
        ) -> DispatchResultWithPostInfo {
            T::FeederOrigin::ensure_origin(origin)?;
            <Pallet<T> as EmergencyPriceFeeder<CurrencyId, OraclePrice>>::reset_emergency_price(
                currency_id,
            );
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    // get emergency price, the timestamp is zero
    fn get_emergency_price(currency_id: &CurrencyId) -> Option<PriceDetail> {
        if let Some(price) = Self::emergency_price(currency_id) {
            Some((price.into_inner(), 0))
        } else {
            None
        }
    }
}

impl<T: Config> PriceFeeder for Pallet<T> {
    /// Get price and timestamp by currency_id
    /// Timestamp is zero and the currency is not stable currency means the price is emergency price
    fn get_price(currency_id: &CurrencyId) -> Option<PriceDetail> {
        // if is stable currency, return fixed price
        if *currency_id == T::GetStableCurrencyId::get() {
            Some((T::StableCurrencyFixedPrice::get().into_inner(), 0))
        } else {
            // if emergency price exists, return it, otherwise return latest price from oracle.
            Self::get_emergency_price(currency_id).or_else(|| {
                T::Source::get_no_op(&currency_id)
                    .and_then(|price| Some((price.value.into_inner(), price.timestamp)))
            })
        }
    }
}

impl<T: Config> EmergencyPriceFeeder<CurrencyId, OraclePrice> for Pallet<T> {
    /// Set emergency price
    fn set_emergency_price(currency_id: CurrencyId, price: OraclePrice) {
        // set price direct
        EmergencyPrice::<T>::insert(currency_id, price.clone());
        <Pallet<T>>::deposit_event(Event::SetEmergencyPrice(currency_id, price));
    }

    /// Reset emergency price
    fn reset_emergency_price(currency_id: CurrencyId) {
        EmergencyPrice::<T>::remove(currency_id);
        <Pallet<T>>::deposit_event(Event::ResetEmergencyPrice(currency_id));
    }
}
