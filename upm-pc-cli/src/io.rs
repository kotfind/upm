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

const READ_TIMEOUT: Duration = Duration::from_millis(200);

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
    tx: EndpointWrite<Bulk>,
    rx: EndpointRead<Bulk>,
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
        let bytes_len = bytes.len() as u64;
        self.tx.write_all(&bytes_len.to_be_bytes()).await?;
        self.tx.write_all(bytes).await?;
        self.tx.flush().await?;
        Ok(())
    }

    pub async fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let msg_len: usize;
        {
            let mut len_buf = [0u8; size_of::<u64>()];
            self.rx.read_exact(&mut len_buf).await?;

            msg_len = u64::from_be_bytes(len_buf) as usize;
        }

        let mut bytes = vec![0u8; msg_len];
        tokio::time::timeout(READ_TIMEOUT, async {
            self.rx.read_exact(bytes.as_mut_slice()).await
        })
        .await
        .map_err(|_| Error::ReadTimeout)??;

        Ok(bytes)
    }
}
