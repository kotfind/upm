use core::error;

use embassy_rp::{
    peripherals::USB,
    usb::{Endpoint, In, Out},
};
use embassy_time::Duration;
use embassy_usb_driver::{Endpoint as _, EndpointError, EndpointIn, EndpointOut};
use generic_array::{ArrayLength, GenericArray};
use log::info;
use minicbor::{Decode, Encode, encode::Write};
use thiserror::Error;
use upm_common::info::USB_PACKET_SIZE;

use crate::gvec::{self, GVec};

const READ_TIMEOUT: Duration = Duration::from_millis(200);

#[derive(Error, Debug)]
pub enum Error {
    #[error("endpoint error")]
    Endpoint(EndpointError),

    #[error("failed to read packet size")]
    Size,

    #[error("a buffer is to small to read a message")]
    BufferTooSmall,

    #[error("reading timed out")]
    ReadTimeout,

    #[error("cbor encode error")]
    CborEncode(#[from] minicbor::encode::Error<gvec::CapacityError>),

    #[error("cbor decode error")]
    CborDecode(#[from] minicbor::decode::Error),
}

impl From<EndpointError> for Error {
    fn from(e: EndpointError) -> Self {
        Self::Endpoint(e)
    }
}

pub struct Io<'a> {
    // fixme pub
    pub tx: Endpoint<'a, USB, In>,
    pub rx: Endpoint<'a, USB, Out>,
}

impl<'a> Io<'a> {
    pub fn new(
        usb_builder: &mut embassy_usb::Builder<'a, embassy_rp::usb::Driver<'a, USB>>,
    ) -> Self {
        let mut func = usb_builder.function(0xFF, 0, 0);
        let mut iface = func.interface();
        let mut alt = iface.alt_setting(0xFF, 0, 0, None);

        let tx = alt.endpoint_bulk_in(None, USB_PACKET_SIZE as u16);
        let rx = alt.endpoint_bulk_out(None, USB_PACKET_SIZE as u16);

        Self { tx, rx }
    }

    pub async fn init(&mut self) {
        self.tx.wait_enabled().await;
        self.rx.wait_enabled().await;
    }

    #[allow(non_camel_case_types)] // typenum types
    pub async fn write_cbor<T: Encode<()>, CBOR_MAX_LEN: ArrayLength>(
        &mut self,
        item: &T,
    ) -> Result<(), Error> {
        let mut bytes = GVec::<u8, CBOR_MAX_LEN>::default();
        minicbor::encode(item, &mut bytes)?;

        self.write_bytes(&bytes).await?;

        Ok(())
    }

    #[allow(non_camel_case_types)] // typenum types
    pub async fn read_cbor<T: for<'b> Decode<'b, ()>, CBOR_MAX_LEN: ArrayLength>(
        &mut self,
    ) -> Result<T, Error> {
        let mut bytes = GenericArray::<u8, CBOR_MAX_LEN>::default();
        let data = self.read_bytes(&mut bytes).await.map(|len| &bytes[..len])?;

        let item = minicbor::decode(&bytes)?;

        Ok(item)
    }

    pub async fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let mut bytes_wrote_total = 0;

        loop {
            let bytes_to_write = (bytes.len() - bytes_wrote_total).min(USB_PACKET_SIZE);

            if bytes_to_write == 0 {
                self.tx.write(&[]).await?;
                break;
            }

            self.tx
                .write(&bytes[bytes_wrote_total..bytes_wrote_total + bytes_to_write])
                .await?;
            bytes_wrote_total += bytes_to_write;

            if bytes_to_write < USB_PACKET_SIZE {
                break;
            }
        }

        Ok(())
    }

    pub async fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<usize, Error> {
        let mut bytes_read_total = 0;

        loop {
            let mut buf = [0u8; USB_PACKET_SIZE];
            let bytes_read = self.rx.read(&mut buf).await?;

            if bytes_read_total + bytes_read > bytes.len() {
                return Err(Error::BufferTooSmall);
            }

            bytes[bytes_read_total..bytes_read_total + bytes_read]
                .clone_from_slice(&buf[..bytes_read]);

            bytes_read_total += bytes_read;

            if bytes_read < USB_PACKET_SIZE {
                break;
            }
        }

        Ok(bytes_read_total)
    }
}
