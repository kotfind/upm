use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::{String, Vec};
use minicbor::{Decode, Encode};
use nameof::name_of;
use rand::CryptoRng;
use rekv::{Entity, Id, Rtx};
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

    #[n(2)]
    ChaCha20Poly1305Key(
        #[n(0)]
        #[cbor(with = "::upm_common::util::garr_cbor")]
        chacha20poly1305::Key,
    ),

    #[n(3)]
    K256Key(
        #[n(0)]
        #[cbor(with = "::upm_common::util::k256_signing_key_cbor")]
        k256::ecdsa::SigningKey,
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
            KeyKind::ChaCha20Poly1305Key(key) => Self::ChaCha20Poly1305Key(key),
            KeyKind::K256Key(key) => Self::K256Key(key),
        }
    }
}

impl From<KeyRecordKind> for KeyKind {
    fn from(kind: KeyRecordKind) -> Self {
        match kind {
            KeyRecordKind::Bytes(bytes) => Self::Bytes(bytes),
            KeyRecordKind::ChaCha20Poly1305Key(key) => Self::ChaCha20Poly1305Key(key),
            KeyRecordKind::K256Key(key) => Self::K256Key(key),
        }
    }
}

pub async fn get_key_record_by_name<'a, F: Flash, M: RawMutex>(
    name: &str,
    rtx: &Rtx<'a, F, M>,
) -> Result<Option<KeyRecord>, rekv::Error<F>> {
    let mut records = rtx.read_all::<KeyRecord>().await?;

    while let Some(record) = records.next().await? {
        if record.name == name {
            return Ok(Some(record));
        }
    }

    Ok(None)
}
