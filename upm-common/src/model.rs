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
    ChaCha20Poly1305Key(
        #[n(0)]
        #[cbor(with = "crate::util::garr_cbor")]
        chacha20poly1305::Key,
    ),

    #[n(3)]
    K256Key(
        #[n(0)]
        #[cbor(with = "crate::util::k256_signing_key_cbor")]
        k256::ecdsa::SigningKey,
    ),
}
