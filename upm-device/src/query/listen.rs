use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use upm_common::Req;

use crate::query::QueryContext;
// use crate::query::write_plain;

pub async fn listen<'a, F: Flash, M: RawMutex>(ctx: &mut QueryContext<'a, F, M>) -> ! {
    loop {
        let msg = ctx.io.listen().await.unwrap();

        match msg {
            Req::WritePlain(r) => {} //  write_plain::process(ctx, r).await,
        }
    }
}
