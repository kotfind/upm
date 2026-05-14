use ekv::{Database, flash::Flash};
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig;
use embassy_rp::{
    Peri,
    gpio::{Level, Output},
    peripherals::{PIN_21, SPI0},
    spi::{self, Async, Spi},
};
use embassy_sync::{
    blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex},
    mutex::Mutex,
};
use embassy_time::Timer;
use rekv::Db;
use w25::Memory;

pub async fn init(
    bus: &Mutex<CriticalSectionRawMutex, Spi<'static, SPI0, Async>>,
    cs_pin: Peri<'static, PIN_21>,
    do_format: bool,
) -> Db<impl Flash, impl RawMutex> {
    let cs = Output::new(cs_pin, Level::High);
    let device = SpiDeviceWithConfig::new(bus, cs, {
        let mut config = spi::Config::default();
        config.frequency = 10_000_000;
        config
    });

    Timer::after_millis(1000).await;

    let mut memory = Memory::new(device);

    memory.reset().await.unwrap();
    memory.check_jedec_id().await.unwrap();

    let kv_db = Database::<_, CriticalSectionRawMutex>::new(memory, {
        let mut config = ekv::Config::default();
        config.random_seed = 0; // seems to misbehave with actually random seed
        config
    });

    if do_format {
        kv_db.format().await.unwrap();
    } else {
        kv_db.mount().await.unwrap();
    }

    Db::new(kv_db)
}
