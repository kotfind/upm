use core::fmt::Write;
use core::panic::PanicInfo;

use embassy_rp::{
    Peripherals,
    gpio::{Level, Output},
    uart::{self, Uart},
    watchdog::Watchdog,
};
use embassy_time::Duration;
use heapless::String;
use static_cell::StaticCell;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    let p = unsafe { Peripherals::steal() };

    // send message
    {
        static BUFFER: StaticCell<String<1024>> = StaticCell::new();
        let buf = BUFFER.init_with(String::new);

        let _ = write!(buf, "{}", info);

        let mut uart = Uart::new_blocking(p.UART0, p.PIN_16, p.PIN_17, uart::Config::default());
        let _ = uart.blocking_write(buf.as_bytes());
        let _ = uart.blocking_write(b"\r\n");
        let _ = uart.blocking_flush();
    }
    cortex_m::asm::delay(1_000_000);

    // reboot
    {
        let mut wd = Watchdog::new(p.WATCHDOG);
        wd.start(Duration::from_micros(1));

        loop {
            cortex_m::asm::wfi();
        }
    }
}
