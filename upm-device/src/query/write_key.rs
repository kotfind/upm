use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use log::info;
use rand::CryptoRng;
use upm_common::{req::WriteKeyReq, resp::WroteKeyResp};

use crate::{
    db::KeyRecord,
    query::{QueryContext, QueryResult},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: WriteKeyReq,
) -> QueryResult {
    let mut wtx = ctx.db.wtx().await?;

    let record = KeyRecord::from_req(req, wtx.new_id()?, ctx.rng);

    wtx.write(&record).await?;

    wtx.commit().await?;

    info!("wrote a record with id={}", record.id.to_inner());
    ctx.io
        .send(WroteKeyResp {
            id: record.id.to_inner(),
        })
        .await?;

    Ok(())
}
