use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use rand::CryptoRng;
use upm_common::{req::GetKeyMetaReq, resp::GotKeyMetaResp};

use crate::{
    db,
    query::{
        QueryContext,
        error::{QueryError, QueryResult},
    },
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: GetKeyMetaReq,
) -> QueryResult {
    let rtx = ctx.db.rtx().await;

    let Some(record) = db::get_key_record_by_name(&req.name, &rtx).await? else {
        let mut msg = String::new();
        let _ = write!(msg, "record with name `{}` does not exist", req.name);
        return Err(QueryError::Custom { msg });
    };

    ctx.io
        .send(GotKeyMetaResp {
            passwd_hint: record.passwd_hint,
        })
        .await?;

    Ok(())
}
