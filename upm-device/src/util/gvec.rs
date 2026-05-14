#![allow(non_camel_case_types)] // for ArrayLength types

use core::{
    fmt::{self, Debug},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    slice,
};

use generic_array::{ArrayLength, GenericArray};
use minicbor::{Decode, Encode};

pub(crate) struct GVec<T, CAP: ArrayLength> {
    data: GenericArray<MaybeUninit<T>, CAP>,
    len: usize,
}

impl<T, CAP: ArrayLength> Default for GVec<T, CAP> {
    fn default() -> Self {
        Self {
            data: GenericArray::uninit(),
            len: 0,
        }
    }
}

impl<T, CAP: ArrayLength> GVec<T, CAP> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_garr<CAP_>(buf: &GenericArray<T, CAP_>, len: usize) -> Self
    where
        T: Clone,
        CAP_: ArrayLength,
    {
        let mut this = Self::new();
        this.extend(&buf[..len]);
        this
    }

    pub fn extend(&mut self, arr: impl AsRef<[T]>)
    where
        T: Clone,
    {
        self.try_extend(arr).expect("failed to extend")
    }

    pub fn try_extend(&mut self, arr: impl AsRef<[T]>) -> Result<(), CapacityError>
    where
        T: Clone,
    {
        let arr = arr.as_ref();

        if self.len + arr.len() > CAP::USIZE {
            return Err(CapacityError);
        }

        for it in arr {
            self.data[self.len].write(it.clone());
            self.len += 1;
        }

        Ok(())
    }

    pub fn try_push(&mut self, item: T) -> Result<(), CapacityError> {
        if self.len + 1 > CAP::USIZE {
            return Err(CapacityError);
        }

        self.data[self.len].write(item);
        self.len += 1;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone, CAP: ArrayLength> Clone for GVec<T, CAP> {
    fn clone(&self) -> Self {
        let mut other = Self::new();
        other.extend(self.deref());
        other
    }
}

impl<T: Debug, CAP: ArrayLength> fmt::Debug for GVec<T, CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<CAP: ArrayLength> minicbor::encode::Write for GVec<u8, CAP> {
    type Error = CapacityError;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.try_extend(buf)
    }
}

impl<CAP: ArrayLength> AsRef<[u8]> for GVec<u8, CAP> {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl<CAP: ArrayLength> AsMut<[u8]> for GVec<u8, CAP> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
}

impl<CAP: ArrayLength> chacha20poly1305::aead::Buffer for GVec<u8, CAP> {
    fn extend_from_slice(&mut self, other: &[u8]) -> chacha20poly1305::aead::Result<()> {
        self.try_extend(other)
            .map_err(|_| chacha20poly1305::aead::Error)
    }

    fn truncate(&mut self, len: usize) {
        if self.len < len {
            panic!("truncate shouldn't increase length");
        }

        self.len = len;
    }
}

impl<T, CAP: ArrayLength> Drop for GVec<T, CAP> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe { self.data[i].assume_init_drop() };
        }
    }
}

impl<T, CAP: ArrayLength> Deref for GVec<T, CAP> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        let len = self.len();
        let ptr = self.data[..len].as_ptr() as *const T;
        unsafe { slice::from_raw_parts(ptr, self.len()) }
    }
}

impl<T, CAP: ArrayLength> DerefMut for GVec<T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = self.len();
        let ptr = self.data[..len].as_mut_ptr() as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, self.len()) }
    }
}

impl<T, CAP: ArrayLength, CTX> Encode<CTX> for GVec<T, CAP>
where
    T: Encode<CTX>,
{
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut CTX,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.array(self.len as u64)?;
        for elem in self.iter() {
            e.encode_with(elem, ctx)?;
        }
        Ok(())
    }
}

impl<'a, T, CAP: ArrayLength, CTX> Decode<'a, CTX> for GVec<T, CAP>
where
    T: Decode<'a, CTX>,
{
    fn decode(
        d: &mut minicbor::Decoder<'a>,
        ctx: &mut CTX,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut this = Self::new();
        let arr = d.array_iter_with::<CTX, T>(ctx)?;

        for elem in arr {
            let elem = elem?;
            this.try_push(elem)
                .map_err(|_| minicbor::decode::Error::message("CapacityError"))?;
        }

        Ok(this)
    }
}

// -------------------- Capacity Error --------------------

#[derive(Debug)]
pub struct CapacityError;

impl fmt::Display for CapacityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for CapacityError {}
