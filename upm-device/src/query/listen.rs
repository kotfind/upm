use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use rand::CryptoRng;
use upm_common::Req;

use crate::query::{QueryContext, write_key};

pub async fn listen<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
) -> ! {
    loop {
        let msg = ctx.io.listen().await.unwrap();

        match msg {
            Req::WriteKey(r) => write_key::process(ctx, r).await,
        }
    }
}
