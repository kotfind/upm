use derive_more::From;
use heapless::{String, Vec};
use minicbor::{Decode, Encode};

// would be nice to use something like this instead:
//     https://github.com/twittner/minicbor/pull/56/
#[allow(non_camel_case_types)]
pub type REQ_CBOR_MAX_LEN = typenum::U1024;

/// A request is a message from a PC to a device.
#[derive(Encode, Decode, From)]
#[allow(clippy::large_enum_variant)]
pub enum Req {
    #[n(1)]
    WritePlain(#[n(0)] WritePlainReq),
}

#[derive(Encode, Decode)]
pub struct WritePlainReq {
    #[n(0)]
    #[cbor(with = "minicbor_adapters")]
    pub name: String<64>,

    #[n(1)]
    #[cbor(with = "minicbor_adapters")]
    pub data: Vec<u8, 1024>, // 1 Kb
}
