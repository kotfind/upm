use cortex_m_rt::{ExceptionFrame, exception};
use embassy_rp::{Peripherals, watchdog::Watchdog};
use embassy_time::Duration;

#[exception]
unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
    let p = unsafe { Peripherals::steal() };

    {
        let mut wd = Watchdog::new(p.WATCHDOG);
        wd.start(Duration::from_micros(1));

        loop {
            cortex_m::asm::wfi();
        }
    }
}
