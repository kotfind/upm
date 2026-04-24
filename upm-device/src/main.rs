#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use log::info;
use upm_common::msg::{MSG_CBOR_MAX_LEN, Msg};

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

    let msg = io.read_cbor::<Msg, MSG_CBOR_MAX_LEN>().await.unwrap();
    info!("{}", msg.text);

    io.write_cbor::<_, MSG_CBOR_MAX_LEN>(&Msg {
        text: "1!".try_into().unwrap(),
    })
    .await
    .unwrap();

    let msg = io.read_cbor::<Msg, MSG_CBOR_MAX_LEN>().await.unwrap();
    info!("{}", msg.text);

    io.write_cbor::<_, MSG_CBOR_MAX_LEN>(&Msg {
        text: "2!".try_into().unwrap(),
    })
    .await
    .unwrap();

    let msg = io.read_cbor::<Msg, MSG_CBOR_MAX_LEN>().await.unwrap();
    info!("{}", msg.text);

    io.write_cbor::<_, MSG_CBOR_MAX_LEN>(&Msg {
        text: "3!".try_into().unwrap(),
    })
    .await
    .unwrap();
}
