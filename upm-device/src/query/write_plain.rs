use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::{Duration, Timer};
use log::info;
use upm_common::{req::WritePlainReq, resp::WrotePlainResp};

use crate::{db::PlainRecord, query::QueryContext};

pub async fn process<'a, F: Flash, M: RawMutex>(
    ctx: &mut QueryContext<'a, F, M>,
    req: WritePlainReq,
) {
    let mut wtx = ctx.db.wtx().await.unwrap();

    let record = PlainRecord {
        id: wtx.new_id().unwrap(),
        name: req.name,
        data: req.data,
    };

    Timer::after_millis(100).await;
    wtx.write(&record).await.unwrap();

    let _ = embassy_time::with_timeout(Duration::from_secs(1), async {
        wtx.commit().await.unwrap();
    })
    .await;

    info!("wrote a record with id={}", record.id.to_inner());
    ctx.io
        .send(WrotePlainResp {
            id: record.id.to_inner(),
        })
        .await
        .unwrap();
}
