use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::{String, Vec};
use minicbor::{Decode, Encode};
use nameof::name_of;
use rekv::{Entity, Id, Rtx};
use typenum::U2048;
use upm_common::model::KeyKind;

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
    ChaCha20Poly1305(
        #[n(0)]
        #[cbor(with = "::upm_common::util::garr_cbor")]
        chacha20poly1305::Key,
    ),

    #[n(3)]
    K256(
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

impl From<KeyKind> for KeyRecordKind {
    fn from(kind: KeyKind) -> Self {
        match kind {
            KeyKind::Bytes(bytes) => Self::Bytes(bytes),
            KeyKind::ChaCha20Poly1305(key) => Self::ChaCha20Poly1305(key),
            KeyKind::K256(key) => Self::K256(key),
        }
    }
}

impl From<KeyRecordKind> for KeyKind {
    fn from(kind: KeyRecordKind) -> Self {
        match kind {
            KeyRecordKind::Bytes(bytes) => Self::Bytes(bytes),
            KeyRecordKind::ChaCha20Poly1305(key) => Self::ChaCha20Poly1305(key),
            KeyRecordKind::K256(key) => Self::K256(key),
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
