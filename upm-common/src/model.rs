use core::fmt;

use heapless::Vec;
use minicbor::{Decode, Encode};

#[derive(Encode, Decode)]
#[allow(clippy::large_enum_variant)]
pub enum KeyKind {
    #[n(0)]
    Bytes(
        #[n(0)]
        #[cbor(with = "::minicbor_adapters")]
        Vec<u8, 1024>,
    ),

    #[n(2)]
    ChaCha20Poly1305(
        #[n(0)]
        #[cbor(with = "crate::util::garr_cbor")]
        chacha20poly1305::Key,
    ),

    #[n(3)]
    K256(
        #[n(0)]
        #[cbor(with = "crate::util::k256_signing_key_cbor")]
        k256::ecdsa::SigningKey,
    ),
}

#[derive(Encode, Decode, Clone, Copy)]
pub enum KeyTy {
    #[n(0)]
    Bytes,

    #[n(1)]
    ChaCha20Poly1305,

    #[n(2)]
    K256,
}

impl fmt::Display for KeyTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bytes => "Custom",
            Self::ChaCha20Poly1305 => "Symmetrical",
            Self::K256 => "Asymmetrical",
        };

        write!(f, "{s}")
    }
}

impl KeyTy {
    pub const ALL: &[Self] = &[Self::Bytes, Self::ChaCha20Poly1305, Self::K256];
}
