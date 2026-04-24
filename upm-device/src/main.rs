#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use log::info;

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
            .map(|len| &mut buf[..len])
            .unwrap();

        info!("read {} bytes", data.len());

        for i in 0..data.len() / 2 {
            data.swap(i, data.len() - i - 1);
        }

        io.write_bytes(data).await.unwrap();
    }
}
