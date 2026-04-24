use std::{error::Error, process, time::Duration};

use nusb::transfer::{Bulk, In, Out};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, stdout},
    task,
};

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => (),
        Err(e) => {
            let mut e: &dyn Error = &e;

            loop {
                eprintln!("- {e}");

                if let Some(next_e) = e.source() {
                    e = next_e;
                } else {
                    process::exit(1);
                }
            }
        }
    }
}

async fn run() -> Result<(), nusb::Error> {
    let di = nusb::list_devices()
        .await?
        .find(|d| {
            use upm_common::info::*;
            d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID
        })
        .expect("no device found");

    let device = di.open().await?;
    let iface = device.claim_interface(0).await?;

    let mut write_ep = iface.endpoint::<Bulk, Out>(0x01)?.writer(1024);
    let mut read_ep = iface.endpoint::<Bulk, In>(0x81)?.reader(1024);

    task::spawn(async move {
        loop {
            let mut buf = [0u8; 1024];
            let data = tokio::io::stdin()
                .read(&mut buf)
                .await
                .map(|n| &buf[..n])
                .expect("failed to read input");

            write_ep.write_all(data).await.expect("failed to write");
            write_ep.flush().await.expect("flush failed");
        }
    });

    task::spawn(async move {
        loop {
            let mut buf = [0u8; 5];
            let data = read_ep
                .read(&mut buf)
                .await
                .map(|n| &buf[..n])
                .expect("failed to read data");

            let s = str::from_utf8(data).expect("failed to parse string");
            print!("{s}");
            stdout().flush().await.expect("failed to flush");
        }
    });

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
