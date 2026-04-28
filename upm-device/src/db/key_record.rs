use heapless::{String, Vec};
use minicbor::{Decode, Encode};
use nameof::name_of;
use rekv::{Entity, Id};
use typenum::U2048;

use crate::enc::PasswdEnc;

#[derive(Encode, Decode)]
pub struct KeyRecord {
    #[n(0)]
    pub id: Id<KeyRecord>,

    #[n(1)]
    #[cbor(with = "minicbor_adapters")]
    pub name: String<64>,

    #[n(2)]
    #[cbor(with = "minicbor_adapters")]
    pub passwd_hint: String<64>,

    #[n(3)]
    pub data: PasswdEnc<KeyRecordKind, U2048>,
}

#[derive(Encode, Decode)]
#[allow(clippy::large_enum_variant)]
pub enum KeyRecordKind {
    #[n(0)]
    Bytes(
        #[n(0)]
        #[cbor(with = "minicbor_adapters")]
        Vec<u8, 1024>,
    ),
    #[n(1)]
    Text(
        #[n(0)]
        #[cbor(with = "minicbor_adapters")]
        String<256>,
    ),

    #[n(2)]
    ChaCha20Poly1305Key(
        #[n(0)]
        #[cbor(with = "crate::util::garr_cbor")]
        chacha20poly1305::Key,
    ),
}

impl Entity for KeyRecord {
    type CBOR_MAX_LEN = typenum::U2048;

    const RAW_TABLE_ID: u8 = 1;

    const DEBUG_NAME: &str = name_of!(type KeyRecord);

    fn id(&self) -> Id<Self> {
        self.id
    }
}
