#[derive(Clone, Debug, Interop)]
#[rs_type(RPriceDataField)]
#[derive_ReprC]
#[repr(C)]
pub enum MarketType {
    Unknown,
    Spot,
    LinearFuture,
    InverseFuture,
    LinearSwap,
    InverseSwap,

    AmericanOption,
    EuropeanOption,

    QuantoFuture,
    QuantoSwap,

    Move,
    #[serde(rename = "bvol")]
    #[allow(clippy::upper_case_acronyms)]
    BVOL,
}