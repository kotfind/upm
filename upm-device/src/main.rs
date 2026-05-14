#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{
    clocks::RoscRng,
    gpio::{Level, Output},
    spi::{self, Spi},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Timer;

use crate::{confirm_io::ConfirmIo, query::QueryContext};

mod blink;
mod confirm_io;
mod db;
mod enc;
mod hard_fault;
mod io;
mod panic;
mod query;
mod usb;
mod util;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // spawner.spawn(blink::init(p.PIN_25)).unwrap();

    let memory_bus = Mutex::<CriticalSectionRawMutex, _>::new(Spi::new(
        p.SPI0,
        p.PIN_18,
        p.PIN_19,
        p.PIN_20,
        p.DMA_CH0,
        p.DMA_CH1,
        spi::Config::default(),
    ));
    let memory_cs_pin = p.PIN_21;

    let (io, db) = join(
        usb::init(spawner, p.USB),
        db::init(&memory_bus, memory_cs_pin, true),
    )
    .await;

    let mut confirm_io = ConfirmIo::new(p.PIN_5, p.PIN_6);
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        confirm_io.confirm().await;
        led.toggle();
        Timer::after_secs(1).await;
    }

    // let rng = &mut RoscRng;
    //
    // query::listen(&mut QueryContext { db, io, rng }).await;
}
