use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use log::error;
use rand::CryptoRng;
use upm_common::Req;

use crate::query::{QueryContext, write_key};

pub async fn listen<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
) -> ! {
    loop {
        let msg = ctx.io.listen().await.unwrap();

        let query_result = match msg {
            Req::WriteKey(r) => write_key::process(ctx, r).await,
        };

        if let Err(e) = query_result
            && let Err(er) = ctx.io.send(e).await
        {
            error!("failed to send query error: {er}");
        }
    }
}
