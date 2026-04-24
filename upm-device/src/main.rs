#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use log::info;
use upm_common::{Req::Blink, req::BlinkReq, resp::BlinkEndedResp};

mod gvec;
mod io;
mod panic;
mod usb;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut io = usb::init(spawner, p.USB).await;

    Timer::after_secs(1).await;

    io.init().await;

    let mut led = Output::new(p.PIN_25, Level::Low);

    match io.listen().await.unwrap() {
        Blink(BlinkReq { n }) => {
            info!("going to blink {n} times");
            for _ in 0..2 * n {
                led.toggle();
                Timer::after_millis(300).await;
            }
            io.send(BlinkEndedResp { n }).await.unwrap();
        }
    }
}
