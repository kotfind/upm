use crate::{db::KeyRecordKind, query::QueryError};
use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use k256::{
    ecdsa::{Signature, signature::Signer},
    sha2::{Digest, Sha256},
};
use rand::CryptoRng;
use upm_common::{req::SignDataReq, resp::SignedDataResp};

use crate::{
    db,
    query::{QueryContext, error::QueryResult},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: SignDataReq,
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

    let KeyRecordKind::K256(key) = kind else {
        return Err(QueryError::Custom {
            msg: "only asymmetric keys are supported for this operation"
                .try_into()
                .unwrap(),
        });
    };

    let hash = Sha256::digest(&req.data);
    let sgn: Signature = key.sign(&hash);

    ctx.io.send(SignedDataResp { sgn }).await?;

    Ok(())
}
