use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use log::info;
use rand::CryptoRng;
use rekv::Rtx;
use upm_common::{req::WriteKeyReq, resp::WroteKeyResp};

use crate::{
    db::KeyRecord,
    query::{QueryContext, QueryResult, error::QueryError},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: WriteKeyReq,
) -> QueryResult {
    let mut wtx = ctx.db.wtx().await?;

    {
        let rtx = ctx.db.rtx().await;

        if let Some(old_record) = get_key_record_by_name(&req.name, &rtx).await? {
            let mut msg = String::new();
            let _ = write!(
                msg,
                "record with name `{}` already exists (id = {:?})",
                req.name, old_record.id
            );
            return Err(QueryError::Custom { msg });
        }
    }

    let record = KeyRecord::from_req(req, wtx.new_id()?, ctx.rng);
    wtx.write(&record).await?;
    wtx.commit().await?;

    info!("wrote a record with id={:?}", record.id);
    ctx.io
        .send(WroteKeyResp {
            id: record.id.to_inner(),
        })
        .await?;

    Ok(())
}

pub async fn get_key_record_by_name<'a, F: Flash, M: RawMutex>(
    name: &str,
    rtx: &Rtx<'a, F, M>,
) -> Result<Option<KeyRecord>, QueryError> {
    let mut records = rtx.read_all::<KeyRecord>().await?;

    while let Some(record) = records.next().await? {
        if record.name == name {
            return Ok(Some(record));
        }
    }

    Ok(None)
}
