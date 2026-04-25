use derive_more::From;
use heapless::String;
use minicbor::{Decode, Encode};

// would be nice to use something like this instead:
//     https://github.com/twittner/minicbor/pull/56/
#[allow(non_camel_case_types)]
pub type RESP_CBOR_MAX_LEN = typenum::U1024;

/// A response is a message from a device to a PC.
#[derive(Encode, Decode, From)]
#[allow(clippy::large_enum_variant)]
pub enum Resp {
    #[n(0)]
    Error(#[n(0)] ErrorResp),

    #[n(1)]
    BlinkEnded(#[n(0)] BlinkEndedResp),

    #[n(2)]
    WrotePlain(#[n(0)] WrotePlainResp),
}

#[derive(Encode, Decode)]
pub struct ErrorResp {
    #[n(0)]
    #[cbor(with = "minicbor_adapters")]
    pub text: String<1024>,
}

#[derive(Encode, Decode)]
pub struct BlinkEndedResp {
    #[n(0)]
    pub n: usize,
}

#[derive(Encode, Decode)]
pub struct WrotePlainResp {
    #[n(0)]
    pub id: u16,
}
