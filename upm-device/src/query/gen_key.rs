use core::fmt::Write;
use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use heapless::{String, Vec};
use k256::ecdsa::SigningKey;
use log::info;
use rand::{CryptoRng, seq::IndexedRandom};
use upm_common::{
    model::{KeyKind, KeyTy},
    req::GenKeyReq,
    resp::GenedKeyResp,
};

use crate::{
    db::{self, KeyRecord},
    enc::PasswdEnc,
    query::{QueryContext, QueryResult, error::QueryError},
};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: GenKeyReq,
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

    let kind = gen_key(req.ty, &mut ctx.rng);

    let record = KeyRecord {
        id: wtx.new_id()?,
        name: req.name,
        passwd_hint: req.passwd_hint,
        kind: PasswdEnc::encrypt(&kind.into(), &req.passwd, &mut ctx.rng).unwrap(),
    };

    wtx.write(&record).await?;
    wtx.commit().await?;

    info!("generated a record with id={:?}", record.id);
    ctx.io
        .send(GenedKeyResp {
            id: record.id.to_inner(),
        })
        .await?;

    Ok(())
}

// TODO: use native methods to generate keys
fn gen_key(ty: KeyTy, rng: &mut impl CryptoRng) -> KeyKind {
    match ty {
        KeyTy::Bytes => {
            let allowed_symbols = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&*+,-./:<=>?^_|~";
            let len = 15;

            let mut bytes = Vec::new();
            for _ in 0..len {
                let c = *allowed_symbols.choose(rng).expect("slice is not empty");
                bytes.push(c).expect("vector is big enough");
            }

            KeyKind::Bytes(bytes)
        }
        KeyTy::ChaCha20Poly1305 => {
            let mut key = chacha20poly1305::Key::default();
            rng.fill_bytes(key.as_mut_slice());
            KeyKind::ChaCha20Poly1305(key)
        }
        KeyTy::K256 => {
            let mut key_bytes = k256::elliptic_curve::generic_array::GenericArray::default();
            rng.fill_bytes(key_bytes.as_mut_slice());
            let key = SigningKey::from_bytes(&key_bytes).expect("the key should be correct");
            KeyKind::K256(key)
        }
    }
}
