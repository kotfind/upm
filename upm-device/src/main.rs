#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{
    clocks::RoscRng,
    spi::{self, Spi},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use heapless::String;
use k256::ecdsa::{
    Signature, SigningKey, VerifyingKey,
    signature::{Signer, Verifier},
};
use log::info;
use minicbor::{Decode, Encode};
use sha2::{Digest, Sha256};

use crate::query::QueryContext;

mod blink;
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

    let rng = &mut RoscRng;

    let priv_key = SigningKey::random(rng);
    let pub_key = VerifyingKey::from(&priv_key);

    let msg = b"Some bytes to sign";
    let hash = Sha256::digest(msg);

    let sgn: Signature = priv_key.sign(&hash);

    info!("sgn: {sgn}");

    info!("{:?}", pub_key.verify(&hash, &sgn));

    info!(
        "{:?}",
        pub_key.verify(&Sha256::digest(b"Is this a hash?"), &sgn)
    );

    // query::listen(&mut QueryContext { db, io, rng }).await;
}

#[derive(Encode, Decode, Debug)]
struct Smth {
    #[n(0)]
    a: i32,

    #[n(1)]
    #[cbor(with = "minicbor_adapters")]
    b: String<64>,
}
