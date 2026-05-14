use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use log::error;
use rand::CryptoRng;
use upm_common::Req;

use crate::query::{
    QueryContext, decode_data, encode_data, gen_key, get_key_data, get_key_meta, list_keys,
    write_key,
};

pub async fn listen<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
) -> ! {
    loop {
        let msg = ctx.io.listen().await.unwrap();

        let query_result = match msg {
            Req::WriteKey(r) => write_key::process(ctx, r).await,
            Req::GetKeyMeta(r) => get_key_meta::process(ctx, r).await,
            Req::GetKeyData(r) => get_key_data::process(ctx, r).await,
            Req::GenKey(r) => gen_key::process(ctx, r).await,
            Req::EncodeData(r) => encode_data::process(ctx, r).await,
            Req::DencodeData(r) => decode_data::process(ctx, r).await,
            Req::ListKeys(r) => list_keys::process(ctx, r).await,
        };

        if let Err(e) = query_result
            && let Err(er) = ctx.io.send(e).await
        {
            error!("failed to send query error: {er}");
        }
    }
}
