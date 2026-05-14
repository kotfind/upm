use crate::{db::KeyRecordKind, query::QueryError};
use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::String;
use k256::ecdsa::{VerifyingKey, signature::Verifier};
use rand::CryptoRng;
use sha2::{Digest, Sha256};
use upm_common::{req::VerifyDataReq, resp::VerifiedDataResp};

use crate::{
    db,
    query::{QueryContext, error::QueryResult},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: VerifyDataReq,
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
    let vkey = VerifyingKey::from(&key);

    let hash = Sha256::digest(&req.data);
    let is_valid = vkey.verify(&hash, &req.sgn).is_ok();

    ctx.io.send(VerifiedDataResp { is_valid }).await?;

    Ok(())
}
