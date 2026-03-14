#![no_std]
#![no_main]

use embassy_executor::Spawner;
use log::info;

mod panic;
mod usb;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut usb_class = usb::init(spawner, p.USB).await;

    loop {
        let mut data = [0u8; 1024];
        let res = usb_class
            .read_packet(&mut data)
            .await
            .map(|data_len| &data[..data_len])
            .unwrap();

        info!("recv: {res:02x?}");
    }
}
