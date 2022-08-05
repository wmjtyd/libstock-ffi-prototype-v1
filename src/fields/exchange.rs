use ::safer_ffi::prelude::*;
use codegen::{deserializer_function, serializer_function};
use strum::FromRepr;
pub use wmjtyd_libstock::data::fields::exchange_type::{
    Exchange as RExchange,
    ExchangeTypeField as RExchangeTypeField,
};

use crate::utils::create_from_owned;

#[derive(Copy, Clone, FromRepr, Debug)]
#[derive_ReprC]
#[repr(u16)]
pub enum Exchange {
    Crypto = 1,
    Ftx = 2,
    Binance = 3,
    Huobi = 8,
    Kucoin = 10,
    Okx = 11,
}

impl From<&RExchangeTypeField> for Exchange {
    fn from(value: &RExchangeTypeField) -> Self {
        Exchange::from_repr(value.0 as u16)
            .expect("The Exchange should be correctly mapped to RExchange.")
    }
}

impl From<&Exchange> for RExchangeTypeField {
    fn from(value: &Exchange) -> Self {
        let r_exchange = RExchange::from_repr(*value as usize)
            .expect("The Exchange should be correctly mapped to RExchange.");

        RExchangeTypeField(r_exchange)
    }
}

create_from_owned!(Exchange, RExchangeTypeField);
serializer_function!(Field<1>, Exchange -> RExchangeTypeField);
deserializer_function!(Field<1>, RExchangeTypeField -> Exchange);
