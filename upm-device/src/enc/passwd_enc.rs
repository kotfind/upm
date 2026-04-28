use core::marker::PhantomData;

use chacha20poly1305::{AeadInPlace, Key, KeyInit, Tag, XChaCha20Poly1305, XNonce};
use generic_array::ArrayLength;
use minicbor::{Decode, Encode};
use pbkdf2::pbkdf2_hmac;
use rand::CryptoRng;
use sha2::Sha256;
use thiserror::Error;

use crate::util::gvec::{CapacityError, GVec};

const SALT_LEN: usize = 16;

// TODO: increase to 100_000 for production
const DEFAULT_ROUND_COUNT: u32 = 10;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cbor encode error")]
    CborEncode(#[from] minicbor::encode::Error<CapacityError>),

    #[error("cbor decode error")]
    CborDecode(#[from] minicbor::decode::Error),

    #[error("crypto error")]
    Crypto(chacha20poly1305::Error),
}

impl From<chacha20poly1305::Error> for Error {
    fn from(e: chacha20poly1305::Error) -> Self {
        Self::Crypto(e)
    }
}

#[derive(Encode, Decode, Debug)]
#[allow(non_camel_case_types)]
pub struct PasswdEnc<T, #[allow(non_camel_case_types)] CBOR_MAX_LEN>
where
    T: Encode<()>,
    for<'a> T: Decode<'a, ()>,
    CBOR_MAX_LEN: ArrayLength,
{
    #[n(0)]
    #[cbor(bound = "CBOR_MAX_LEN:")]
    data: GVec<u8, CBOR_MAX_LEN>,

    #[n(1)]
    #[cbor(with = "crate::util::garr_cbor")]
    auth_tag: Tag,

    #[n(2)]
    #[cbor(with = "crate::util::garr_cbor")]
    nonce: XNonce,

    #[n(3)]
    salt: [u8; SALT_LEN],

    #[n(4)]
    round_count: u32,

    #[cbor(skip)]
    marker: PhantomData<T>,
}

impl<T, #[allow(non_camel_case_types)] CBOR_MAX_LEN> PasswdEnc<T, CBOR_MAX_LEN>
where
    T: Encode<()>,
    for<'a> T: Decode<'a, ()>,
    CBOR_MAX_LEN: ArrayLength,
{
    pub fn encrypt(item: &T, passwd: &[u8], rng: &mut impl CryptoRng) -> Result<Self, Error> {
        let mut salt = [0u8; SALT_LEN];
        rng.fill_bytes(&mut salt);

        let mut key = Key::default();
        let round_count = DEFAULT_ROUND_COUNT;
        pbkdf2_hmac::<Sha256>(passwd, &salt, round_count, &mut key);

        let mut nonce = XNonce::default();
        rng.fill_bytes(&mut nonce);

        let mut data = GVec::<u8, CBOR_MAX_LEN>::new();
        minicbor::encode(item, &mut data)?;

        let cipher = XChaCha20Poly1305::new(&key);
        let auth_tag = cipher.encrypt_in_place_detached(&nonce, &[], &mut data)?;

        Ok(Self {
            data,
            auth_tag,
            nonce,
            salt,
            round_count,
            marker: PhantomData,
        })
    }

    pub fn decrypt(&self, passwd: &[u8]) -> Result<T, Error> {
        let mut key = Key::default();
        pbkdf2_hmac::<Sha256>(passwd, &self.salt, self.round_count, &mut key);

        let mut data = self.data.clone();
        let cipher = XChaCha20Poly1305::new(&key);
        cipher.decrypt_in_place_detached(&self.nonce, &[], &mut data, &self.auth_tag)?;

        let item = minicbor::decode(&data)?;

        Ok(item)
    }
}
