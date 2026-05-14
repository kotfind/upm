use crate::{db::KeyRecordKind, query::QueryError};
use chacha20poly1305::{AeadInPlace, KeyInit, XChaCha20Poly1305, XNonce};
use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use rand::CryptoRng;
use upm_common::{req::EncodeDataReq, resp::EncodedDataResp};

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

    ctx.cfm_io.confirm().await;

    let Ok(kind) = record.kind.decrypt(&req.passwd) else {
        let mut msg = String::new();
        let _ = write!(msg, "failed to decrypt `{}`", req.name);
        return Err(QueryError::Custom { msg });
    };

    let KeyRecordKind::ChaCha20Poly1305(key) = kind else {
        return Err(QueryError::Custom {
            msg: "only symetric keys are supported for this operation"
                .try_into()
                .unwrap(),
        });
    };

    let mut nonce = XNonce::default();
    ctx.rng.fill_bytes(&mut nonce);

    let cipher = XChaCha20Poly1305::new(&key);
    let mut data = req.data;
    let auth_tag = cipher.encrypt_in_place_detached(&nonce, &[], &mut data)?;

    ctx.io
        .send(EncodedDataResp {
            nonce,
            auth_tag,
            data,
        })
        .await?;

    Ok(())
}
