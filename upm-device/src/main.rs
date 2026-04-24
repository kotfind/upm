#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_usb_driver::EndpointOut;
use log::{error, info};

mod io;
mod panic;
mod usb;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut io = usb::init(spawner, p.USB).await;

    Timer::after_secs(1).await;

    io.init().await;

    loop {
        let mut buf = [0u8; 1024];
        let data = io
            .read_bytes(&mut buf)
            .await
            .map(|len| &buf[..len])
            .unwrap();

        info!("read {} bytes", data.len());
    }
}
