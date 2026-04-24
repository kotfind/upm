use core::fmt::Write;

use embassy_executor::Spawner;
use embassy_rp::{
    Peri, bind_interrupts,
    peripherals::USB,
    usb::{self, Endpoint},
};
use embassy_time::Instant;
use embassy_usb::{
    UsbDevice,
    class::cdc_acm::{self, CdcAcmClass},
};
use embassy_usb_logger::Writer;
use log::{Level, LevelFilter, Record};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

pub async fn init(
    spawner: Spawner,
    usb: Peri<'static, USB>,
) -> (
    Endpoint<'static, USB, usb::In>,
    Endpoint<'static, USB, usb::Out>,
) {
    let driver = usb::Driver::new(usb, Irqs);

    let config = {
        use upm_common::info::*;

        let mut c = embassy_usb::Config::new(VENDOR_ID, PRODUCT_ID);
        c.manufacturer = Some(MANUFACTURER_NAME);
        c.product = Some(PRODUCT_NAME);
        c.serial_number = None;
        c
    };

    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        )
    };

    let write_ep;
    let read_ep;
    {
        let mut func = builder.function(0xFF, 0, 0);
        let mut iface = func.interface();
        let mut alt = iface.alt_setting(0xFF, 0, 0, None);
        write_ep = alt.endpoint_bulk_in(None, 64);
        read_ep = alt.endpoint_bulk_out(None, 64);
    }

    let logger_class = {
        static STATE: StaticCell<cdc_acm::State> = StaticCell::new();
        CdcAcmClass::new(&mut builder, STATE.init(cdc_acm::State::new()), 64)
    };

    let usb = builder.build();

    spawner
        .spawn(usb_worker_task(usb))
        .expect("failed to spawn usb_worker_task");
    spawner
        .spawn(usb_logger_task(logger_class))
        .expect("failed to spawn usb_logger_task");

    (write_ep, read_ep)
}

#[embassy_executor::task]
async fn usb_worker_task(mut usb: UsbDevice<'static, usb::Driver<'static, USB>>) -> ! {
    usb.run().await
}

#[embassy_executor::task]
async fn usb_logger_task(class: CdcAcmClass<'static, usb::Driver<'static, USB>>) {
    embassy_usb_logger::with_custom_style!(1024, LevelFilter::Debug, class, format_record).await
}

fn format_record<const N: usize>(record: &Record<'_>, writer: &mut Writer<'_, N>) {
    let secs = Instant::now().as_micros() as f64 / 1_000_000.0;

    let level = match record.level() {
        Level::Error => "ERR",
        Level::Warn => "WRN",
        Level::Info => "INF",
        Level::Debug => "DBG",
        Level::Trace => "TRC",
    };

    let file = record.file().unwrap_or("?");
    let line = record.line().unwrap_or(0);
    let msg = record.args();

    write!(
        writer,
        "[{level}] at {secs:>8.5} in {file}:{line}: {msg}\r\n"
    )
    .expect("failed to format usb_logger message");
}
