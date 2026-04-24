use minicbor::{Decode, Encode};

// would be nice to use something like this instead:
//     https://github.com/twittner/minicbor/pull/56/
#[allow(non_camel_case_types)]
pub type RESP_CBOR_MAX_LEN = typenum::U1024;

/// A response is a message from a device to a PC.
#[derive(Encode, Decode)]
pub enum Resp {
    #[n(0)]
    BlinkEnded(#[n(0)] BlinkEndedResp),
}

#[derive(Encode, Decode)]
pub struct BlinkEndedResp {
    #[n(0)]
    pub n: usize,
}

impl From<BlinkEndedResp> for Resp {
    fn from(value: BlinkEndedResp) -> Self {
        Self::BlinkEnded(value)
    }
}
