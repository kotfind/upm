use minicbor::{Decode, Encode};

// would be nice to use something like this instead:
//     https://github.com/twittner/minicbor/pull/56/
#[allow(non_camel_case_types)]
pub type REQ_CBOR_MAX_LEN = typenum::U1024;

/// A request is a message from a PC to a device.
#[derive(Encode, Decode)]
pub enum Req {
    #[n(0)]
    Blink(#[n(0)] BlinkReq),
}

#[derive(Encode, Decode)]
pub struct BlinkReq {
    #[n(0)]
    pub n: usize,
}

impl From<BlinkReq> for Req {
    fn from(value: BlinkReq) -> Self {
        Self::Blink(value)
    }
}
