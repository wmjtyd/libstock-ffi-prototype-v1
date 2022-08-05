use ::safer_ffi::prelude::*;
use codegen::{deserializer_function, serializer_function, InteropEnum};
use strum::FromRepr;
pub use wmjtyd_libstock::data::fields::exchange_type::{
    Exchange as RExchange,
    ExchangeTypeField as RExchangeTypeField
};

#[derive(Copy, Clone, FromRepr, Debug, InteropEnum)]
#[rs_type(RExchange)]
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

serializer_function!(Field<1>, Exchange -> RExchangeTypeField);
deserializer_function!(Field<1>, RExchangeTypeField -> Exchange);
