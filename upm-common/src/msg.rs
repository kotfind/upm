use heapless::String;
use minicbor::{Decode, Encode};

#[derive(Encode, Decode)]
pub struct Msg {
    #[n(0)]
    #[cbor(with = "minicbor_adapters")]
    pub text: String<64>,
}

// would be nice to use something like this instead:
//     https://github.com/twittner/minicbor/pull/56/
#[allow(non_camel_case_types)]
pub type MSG_CBOR_MAX_LEN = typenum::U1024;
