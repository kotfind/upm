use embassy_rp::{
    Peri,
    gpio::{Level, Output},
    peripherals::PIN_25,
};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn init(pin_25: Peri<'static, PIN_25>) {
    let mut led = Output::new(pin_25, Level::Low);

    loop {
        led.toggle();
        Timer::after_millis(200).await;
    }
}
