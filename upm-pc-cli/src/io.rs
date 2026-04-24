use std::time::Duration;

use nusb::{
    io::{EndpointRead, EndpointWrite},
    transfer::{Bulk, In, Out},
};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const TX_EP_ADDR: u8 = 0x01;
const RX_EP_ADDR: u8 = 0x81;

const TX_BUF_SIZE: usize = 4096;
const RX_BUF_SIZE: usize = 4096;

const READ_TIMEOUT: Duration = Duration::from_millis(5000);

#[derive(Error, Debug)]
pub enum Error {
    #[error("nusb")]
    Nusb(#[from] nusb::Error),

    #[error("std::io")]
    StdIo(#[from] std::io::Error),

    #[error("read timed out")]
    ReadTimeout,
}

pub struct Io {
    // FIXME: pub
    pub tx: EndpointWrite<Bulk>,
    pub rx: EndpointRead<Bulk>,
}

impl Io {
    pub async fn new() -> Result<Self, Error> {
        let di = nusb::list_devices()
            .await?
            .find(|d| {
                use upm_common::info::*;
                d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID
            })
            .expect("no device found");

        let device = di.open().await?;
        let iface = device.claim_interface(0).await?;

        let tx = iface.endpoint::<Bulk, Out>(TX_EP_ADDR)?.writer(TX_BUF_SIZE);
        let rx = iface.endpoint::<Bulk, In>(RX_EP_ADDR)?.reader(RX_BUF_SIZE);

        Ok(Self { tx, rx })
    }

    pub async fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.tx.write_all(bytes).await?;
        self.tx.flush_end_async().await?;
        Ok(())
    }

    pub async fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();

        let mut reader = self.rx.until_short_packet();
        reader.read_to_end(&mut data).await?;
        reader.consume_end().unwrap();

        Ok(data)
    }
}
