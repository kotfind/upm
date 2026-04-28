use heapless::{String, Vec};
use minicbor::{Decode, Encode};
use nameof::name_of;
use rand::CryptoRng;
use rekv::{Entity, Id};
use typenum::U2048;
use upm_common::{model::KeyKind, req::WriteKeyReq};

use crate::enc::PasswdEnc;

#[derive(Encode, Decode)]
pub struct KeyRecord {
    #[n(0)]
    pub id: Id<KeyRecord>,

    #[n(1)]
    #[cbor(with = "::minicbor_adapters")]
    pub name: String<64>,

    #[n(2)]
    #[cbor(with = "::minicbor_adapters")]
    pub passwd_hint: String<64>,

    #[n(3)]
    pub kind: PasswdEnc<KeyRecordKind, U2048>,
}

#[derive(Encode, Decode)]
#[allow(clippy::large_enum_variant)]
pub enum KeyRecordKind {
    #[n(0)]
    Bytes(
        #[n(0)]
        #[cbor(with = "::minicbor_adapters")]
        Vec<u8, 1024>,
    ),
    #[n(1)]
    Text(
        #[n(0)]
        #[cbor(with = "::minicbor_adapters")]
        String<256>,
    ),

    #[n(2)]
    ChaCha20Poly1305Key(
        #[n(0)]
        #[cbor(with = "::upm_common::util::garr_cbor")]
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

impl KeyRecord {
    pub fn from_req(
        WriteKeyReq {
            name,
            passwd_hint,
            passwd,
            kind,
        }: WriteKeyReq,
        id: Id<KeyRecord>,
        rng: &mut impl CryptoRng,
    ) -> Self {
        Self {
            id,
            name,
            passwd_hint,
            kind: PasswdEnc::encrypt(&kind.into(), &passwd, rng).unwrap(),
        }
    }
}

impl From<KeyKind> for KeyRecordKind {
    fn from(kind: KeyKind) -> Self {
        match kind {
            KeyKind::Bytes(bytes) => Self::Bytes(bytes),
            KeyKind::Text(text) => Self::Text(text),
            KeyKind::ChaCha20Poly1305Key(key) => Self::ChaCha20Poly1305Key(key),
        }
    }
}
