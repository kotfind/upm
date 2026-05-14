//! A module to be used in minicbor's `#[cbor(with = "k256_signing_key_cbor")]`

use k256::{ecdsa::SigningKey, elliptic_curve::generic_array::GenericArray};
use minicbor::{Decoder, Encoder};

pub fn encode<W, CTX>(
    this: &SigningKey,
    e: &mut Encoder<W>,
    ctx: &mut CTX,
) -> Result<(), minicbor::encode::Error<W::Error>>
where
    W: minicbor::encode::Write,
{
    let bytes = this.to_bytes();
    e.array(bytes.len() as u64)?;

    for item in bytes {
        e.encode_with(item, ctx)?;
    }

    Ok(())
}

pub fn decode<'a, CTX>(
    d: &mut Decoder<'a>,
    ctx: &mut CTX,
) -> Result<SigningKey, minicbor::decode::Error> {
    let mut bytes = GenericArray::default();

    let arr = d.array_iter_with(ctx)?;
    for (idx, item) in arr.enumerate() {
        bytes[idx] = item?;
    }

    let this = SigningKey::from_bytes(&bytes).unwrap();
    Ok(this)
}
