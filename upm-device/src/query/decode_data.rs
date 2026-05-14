use crate::{db::KeyRecordKind, query::QueryError};
use chacha20poly1305::{AeadInPlace, KeyInit, XChaCha20Poly1305};
use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use rand::CryptoRng;
use upm_common::{req::DecodeDataReq, resp::DecodedDataResp};

use crate::{
    db,
    query::{QueryContext, error::QueryResult},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: DecodeDataReq,
) -> QueryResult {
    let rtx = ctx.db.rtx().await;

    let Some(record) = db::get_key_record_by_name(&req.name, &rtx).await? else {
        let mut msg = String::new();
        let _ = write!(msg, "record with name `{}` does not exist", req.name);
        return Err(QueryError::Custom { msg });
    };

    ctx.cfm_io.confirm().await;

    let Ok(kind) = record.kind.decrypt(&req.passwd) else {
        let mut msg = String::new();
        let _ = write!(msg, "failed to decrypt `{}`", req.name);
        return Err(QueryError::Custom { msg });
    };

    let KeyRecordKind::ChaCha20Poly1305(key) = kind else {
        return Err(QueryError::Custom {
            msg: "only symmetric keys are supported for this operation"
                .try_into()
                .unwrap(),
        });
    };

    let cipher = XChaCha20Poly1305::new(&key);

    let mut data = req.data;

    if cipher
        .decrypt_in_place_detached(&req.nonce, &[], &mut data, &req.auth_tag)
        .is_err()
    {
        return Err(QueryError::Custom {
            msg: "failed to decrypt data: you might've picked the wrong key"
                .try_into()
                .unwrap(),
        });
    }

    ctx.io.send(DecodedDataResp { data }).await?;

    Ok(())
}
