//! A module to be used in minicbor's `#[cbor(with = "...")]`.

// FIXME: use normal generic_array version
use chacha20poly1305::aead::generic_array::{ArrayLength, GenericArray};
use minicbor::{Decode, Decoder, Encode, Encoder};

pub fn encode<T, LEN, W, CTX>(
    this: &GenericArray<T, LEN>,
    e: &mut Encoder<W>,
    ctx: &mut CTX,
) -> Result<(), minicbor::encode::Error<W::Error>>
where
    T: Encode<CTX>,
    LEN: ArrayLength<T>,
    W: minicbor::encode::Write,
{
    e.array(LEN::U64)?;

    for item in this {
        e.encode_with(item, ctx)?;
    }

    Ok(())
}

pub fn decode<'a, T, LEN, CTX>(
    d: &mut Decoder<'a>,
    ctx: &mut CTX,
) -> Result<GenericArray<T, LEN>, minicbor::decode::Error>
where
    T: for<'b> Decode<'b, CTX> + Default,
    LEN: ArrayLength<T>,
{
    let mut this = GenericArray::<T, LEN>::default();

    let arr = d.array_iter_with(ctx)?;
    for (idx, item) in arr.enumerate() {
        this[idx] = item?;
    }

    Ok(this)
}
