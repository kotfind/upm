#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;

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
        io.write_bytes(b"hello").await.unwrap();
        Timer::after_millis(100).await;
    }
}
