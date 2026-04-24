use embassy_rp::{
    peripherals::USB,
    usb::{Endpoint, In, Out},
};
use embassy_time::Duration;
use embassy_usb_driver::{Endpoint as _, EndpointError, EndpointIn, EndpointOut};
use log::info;
use thiserror::Error;
use upm_common::info::USB_PACKET_SIZE;

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

    // pub async fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {}

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
