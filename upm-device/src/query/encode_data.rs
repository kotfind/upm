use crate::query::QueryError;
use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use rand::CryptoRng;
use upm_common::{Req, Resp, req::EncodeDataReq, resp::GotKeyDataResp};

use crate::{
    db,
    query::{QueryContext, error::QueryResult},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: EncodeDataReq,
) -> QueryResult {
    let rtx = ctx.db.rtx().await;

    let Some(record) = db::get_key_record_by_name(&req.name, &rtx).await? else {
        let mut msg = String::new();
        let _ = write!(msg, "record with name `{}` does not exist", req.name);
        return Err(QueryError::Custom { msg });
    };

    let Ok(kind) = record.kind.decrypt(&req.passwd) else {
        let mut msg = String::new();
        let _ = write!(msg, "failed to decrypt `{}`", req.name);
        return Err(QueryError::Custom { msg });
    };

    ctx.io.send(Resp::EncodeDataInitOk).await?;

    loop {
        match ctx.io.listen().await? {
            Req::DataChunk(data) => todo!(),
            Req::EndOfData => break,
            _ => {
                return Err(QueryError::Custom {
                    msg: "got unexpected message".try_into().unwrap(),
                });
            }
        }
    }

    Ok(())
}
