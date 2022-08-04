// use ::safer_ffi::prelude::*;
// use codegen::{Interop, alloc_function};
// use wmjtyd_libstock::data::bbo::BboStructure as RBboStructure;
// use safer_ffi::derive_ReprC;

// use crate::fields::PriceDataField;

// /// The structure of a price data.
// #[derive(Clone, Debug, Interop)]
// #[rs_type(RBboStructure)]
// #[derive_ReprC]
// #[repr(C)]
// pub struct BboMsg {
//     /// 交易所時間戳
//     #[into]
//     pub exchange_timestamp: u64,

//     /// 收到時間戳
//     #[into]
//     pub received_timestamp: u64,

//     /// 交易所類型 (EXCHANGE)
//     pub exchange_type: ExchangeTypeField,

//     /// 市場類型 (MARKET_TYPE)
//     pub market_type: MarketTypeField,

//     /// 訊息類型 (MESSAGE_TYPE)
//     pub message_type: MessageTypeField,

//     /// SYMBOL
//     pub symbol: SymbolPairField,

//     /// 最優賣出報價資訊 (asks)
//     pub asks: PriceDataField,

//     /// 最優買入報價資訊 (bids)
//     pub bids: PriceDataField,

//     /// 資料結尾
//     /// 
//     /// 不需要指定。僅為佔位符號。
//     #[default]
//     pub end: (),
// }

// alloc_function!(PriceDataField);
