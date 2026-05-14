use chacha20poly1305::{Tag, XNonce};
use derive_more::From;
use heapless::{String, Vec};
use minicbor::{Decode, Encode};

use crate::model::KeyKind;

// would be nice to use something like this instead:
//     https://github.com/twittner/minicbor/pull/56/
#[allow(non_camel_case_types)]
pub type RESP_CBOR_MAX_LEN = typenum::U2048;

/// A response is a message from a device to a PC.
#[derive(Encode, Decode, From)]
#[allow(clippy::large_enum_variant)]
pub enum Resp {
    #[n(0)]
    Error(#[n(0)] ErrorResp),

    #[n(2)]
    WroteKey(#[n(0)] WroteKeyResp),

    #[n(3)]
    GotKeyMeta(#[n(0)] GotKeyMetaResp),

    #[n(4)]
    GotKeyData(#[n(0)] GotKeyDataResp),

    #[n(5)]
    GenedKey(#[n(0)] GenedKeyResp),

    #[n(8)]
    EncodedData(#[n(0)] EncodedDataResp),

    #[n(9)]
    DecodedData(#[n(0)] DecodedDataResp),
}

#[derive(Encode, Decode)]
pub struct ErrorResp {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub text: String<128>,
}

#[derive(Encode, Decode)]
pub struct WroteKeyResp {
    #[n(0)]
    pub id: u16,
}

#[derive(Encode, Decode)]
pub struct GotKeyMetaResp {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd_hint: String<64>,
}

#[derive(Encode, Decode)]
pub struct GotKeyDataResp {
    #[n(0)]
    pub kind: KeyKind,
}

#[derive(Encode, Decode)]
pub struct GenedKeyResp {
    #[n(0)]
    pub id: u16,
}

#[derive(Encode, Decode)]
pub struct EncodedDataResp {
    #[n(0)]
    #[cbor(with = "crate::util::garr_cbor")]
    pub nonce: XNonce,

    #[n(1)]
    #[cbor(with = "crate::util::garr_cbor")]
    pub auth_tag: Tag,

    #[n(2)]
    #[cbor(with = "::minicbor_adapters")]
    pub data: Vec<u8, 1024>,
}

#[derive(Encode, Decode)]
pub struct DecodedDataResp {
    #[n(0)]
    #[cbor(with = "::minicbor_adapters")]
    pub data: Vec<u8, 1024>,
}
