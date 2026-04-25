#![no_std]
#![no_main]

use core::hint::black_box;

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{
    Peri,
    gpio::{Level, Output},
    peripherals::PIN_25,
    spi::{self, Spi},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Timer;
use heapless::{String, Vec};
use log::info;
use minicbor::{Decode, Encode};
use nameof::name_of;
use rekv::{Entity, Id};

use crate::{db::PlainRecord, query::QueryContext};

mod blink;
mod db;
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

    // let mut wtx = db.wtx().await.unwrap();
    //
    // let smth = PlainRecord {
    //     id: wtx.new_id().unwrap(),
    //     name: "Hello".try_into().unwrap(),
    //     data: b"Some data".iter().cloned().collect(),
    // };
    //
    // wtx.write(&smth).await.unwrap();
    // wtx.commit().await.unwrap();

    let mut ctx = QueryContext { db, io };

    query::listen(&mut ctx).await;
}

// #[derive(Encode, Decode)]
// pub struct Smth {
//     #[n(0)]
//     pub id: Id<Smth>,
//
//     #[n(1)]
//     #[cbor(with = "minicbor_adapters")]
//     pub name: String<64>,
// }
//
// impl Entity for Smth {
//     type CBOR_MAX_LEN = typenum::U128;
//
//     const RAW_TABLE_ID: u8 = 1;
//
//     const DEBUG_NAME: &str = name_of!(type Smth);
//
//     fn id(&self) -> Id<Self> {
//         self.id
//     }
// }
