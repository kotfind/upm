use embassy_futures::select::select;
use embassy_rp::{
    Peri,
    gpio::{Input, Level, Output, Pin, Pull},
};
use embassy_time::Timer;

pub struct ConfirmIo<'a> {
    btn: Input<'a>,
    led: Output<'a>,
}

impl<'a> ConfirmIo<'a> {
    pub fn new(btn_pin: Peri<'a, impl Pin>, led_pin: Peri<'a, impl Pin>) -> Self {
        let btn = Input::new(btn_pin, Pull::Up);
        let led = Output::new(led_pin, Level::Low);

        Self { btn, led }
    }

    pub async fn confirm(&mut self) {
        select(
            // full click: released -> pressed -> released
            async {
                self.btn.wait_for_high().await;
                self.btn.wait_for_low().await;
                debounce().await;
                self.btn.wait_for_high().await;
                debounce().await;
            },
            async {
                loop {
                    self.led.toggle();
                    Timer::after_millis(100).await;
                }
            },
        )
        .await;

        self.led.set_low();
    }
}

async fn debounce() {
    Timer::after_millis(100).await;
}
