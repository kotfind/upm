use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::{Duration, Timer};
use log::info;
use rand::CryptoRng;
use upm_common::{req::WriteKeyReq, resp::WroteKeyResp};

use crate::{db::KeyRecord, query::QueryContext};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: WriteKeyReq,
) {
    let mut wtx = ctx.db.wtx().await.unwrap();

    let record = KeyRecord::from_req(req, wtx.new_id().unwrap(), ctx.rng);

    Timer::after_millis(100).await;
    wtx.write(&record).await.unwrap();

    let _ = embassy_time::with_timeout(Duration::from_secs(1), async {
        wtx.commit().await.unwrap();
    })
    .await;

    info!("wrote a record with id={}", record.id.to_inner());
    ctx.io
        .send(WroteKeyResp {
            id: record.id.to_inner(),
        })
        .await
        .unwrap();
}
