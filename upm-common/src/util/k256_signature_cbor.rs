//! A module to be used in minicbor's `#[cbor(with = "...")]`.

use k256::{ecdsa::Signature, elliptic_curve::generic_array::GenericArray};
use minicbor::{Decoder, Encoder};

pub fn encode<W, CTX>(
    this: &Signature,
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
) -> Result<Signature, minicbor::decode::Error> {
    let mut bytes = GenericArray::default();

    let arr = d.array_iter_with(ctx)?;
    for (idx, item) in arr.enumerate() {
        bytes[idx] = item?;
    }

    let this = Signature::from_bytes(&bytes).unwrap();
    Ok(this)
}
