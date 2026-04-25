use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::Timer;
use log::info;
use upm_common::Req;

use crate::{
    query::{QueryContext, write_plain},
    util::logging::log_error,
};

pub async fn listen<'a, F: Flash, M: RawMutex>(ctx: &mut QueryContext<'a, F, M>) -> ! {
    loop {
        let msg = ctx.io.listen().await.unwrap();

        Timer::after_millis(100).await;

        info!("HERE");

        match msg {
            Req::Blink(_) => unreachable!(),
            Req::WritePlain(r) => write_plain::process(ctx, r).await,
        }

        // loop {
        //     Timer::after_millis(100).await;
        // }
    }
    // loop {
    //     let msg = match ctx.io.listen().await {
    //         Ok(msg) => msg,
    //         Err(e) => {
    //             info!("HERE");
    //             log_error(&e);
    //             continue;
    //         }
    //     };
    //
    //     info!("HERE HERE");
    //
    //     match msg {
    //         Req::Blink(_) => unreachable!(),
    //         Req::WritePlain(r) => write_plain::process(ctx, r).await,
    //     }
    // }
}
