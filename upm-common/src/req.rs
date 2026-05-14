use derive_more::From;
use heapless::String;
use minicbor::{Decode, Encode};

use crate::model::{DataChunk, KeyKind, KeyTy};

// would be nice to use something like this instead:
//     https://github.com/twittner/minicbor/pull/56/
#[allow(non_camel_case_types)]
pub type REQ_CBOR_MAX_LEN = typenum::U2048;

/// A request is a message from a PC to a device.
#[derive(Encode, Decode, From)]
#[allow(clippy::large_enum_variant)]
pub enum Req {
    #[n(1)]
    WriteKey(#[n(0)] WriteKeyReq),

    #[n(2)]
    GetKeyMeta(#[n(0)] GetKeyMetaReq),

    #[n(3)]
    GetKeyData(#[n(0)] GetKeyDataReq),

    #[n(4)]
    GenKey(#[n(0)] GenKeyReq),

    #[n(5)]
    DataChunk(#[n(0)] DataChunk),

    #[n(6)]
    EndOfData,

    #[n(7)]
    EncodeData(#[n(0)] EncodeDataReq),
}

#[derive(Encode, Decode)]
pub struct WriteKeyReq {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub name: String<64>,

    #[n(1)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd_hint: String<64>,

    #[n(2)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd: String<64>,

    #[n(3)]
    pub kind: KeyKind, // 1 Kb
}

#[derive(Encode, Decode)]
pub struct GetKeyMetaReq {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub name: String<64>,
}

#[derive(Encode, Decode)]
pub struct GetKeyDataReq {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub name: String<64>,

    #[n(2)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd: String<64>,
}

#[derive(Encode, Decode)]
pub struct GenKeyReq {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub name: String<64>,

    #[n(1)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd_hint: String<64>,

    #[n(2)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd: String<64>,

    #[n(3)]
    pub ty: KeyTy,
}

#[derive(Encode, Decode)]
pub struct EncodeDataReq {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub name: String<64>,

    #[n(2)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd: String<64>,
}
