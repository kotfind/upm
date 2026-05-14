use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use rand::CryptoRng;
use upm_common::{req::ListKeysReq, resp::ListedKeyResp};

use crate::{
    db::KeyRecord,
    query::{QueryContext, error::QueryResult},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    ListKeysReq: ListKeysReq,
) -> QueryResult {
    let rtx = ctx.db.rtx().await;

    let mut records = rtx.read_all::<KeyRecord>().await?;
    while let Some(record) = records.next().await? {
        ctx.io
            .send(ListedKeyResp::Key { name: record.name })
            .await?;
    }

    ctx.io.send(ListedKeyResp::EndOfList).await?;

    Ok(())
}
