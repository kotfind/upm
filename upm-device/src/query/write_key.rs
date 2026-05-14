use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use log::info;
use rand::CryptoRng;
use upm_common::{req::WriteKeyReq, resp::WroteKeyResp};

use crate::{
    db::{self, KeyRecord},
    enc::PasswdEnc,
    query::{QueryContext, QueryResult, error::QueryError},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: WriteKeyReq,
) -> QueryResult {
    let mut wtx = ctx.db.wtx().await?;

    {
        let rtx = ctx.db.rtx().await;

        if let Some(old_record) = db::get_key_record_by_name(&req.name, &rtx).await? {
            let mut msg = String::new();
            let _ = write!(
                msg,
                "record with name `{}` already exists (id = {:?})",
                req.name, old_record.id
            );
            return Err(QueryError::Custom { msg });
        }
    }

    let record = KeyRecord {
        id: wtx.new_id()?,
        name: req.name,
        passwd_hint: req.passwd_hint,
        kind: PasswdEnc::encrypt(&req.kind.into(), &req.passwd, &mut ctx.rng).unwrap(),
    };

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
