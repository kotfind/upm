#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_usb_driver::{Endpoint, EndpointIn, EndpointOut};
use log::{error, info};

mod panic;
mod usb;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let (mut write_ep, mut read_ep) = usb::init(spawner, p.USB).await;

    Timer::after_secs(1).await;

    read_ep.wait_enabled().await;
    write_ep.wait_enabled().await;
    info!("Connected");

    loop {
        let mut buf = [0u8; 1024];
        let data = match read_ep.read(&mut buf).await.map(|n| &buf[..n]) {
            Ok(v) => v,
            Err(e) => {
                error!("failed to read: {e:?}");
                continue;
            }
        };

        info!("read {} bytes", data.len());

        if let Err(e) = write_ep.write(data).await {
            error!("failed to write: {e:?}");
            continue;
        }

        Timer::after_secs(1).await;
    }
}
