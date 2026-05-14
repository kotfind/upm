use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use rand::CryptoRng;
use upm_common::req::GetKeyDataReq;

use crate::query::{QueryContext, error::QueryResult};

pub async fn process<'a, F: Flash, M: RawMutex, R: CryptoRng>(
    ctx: &mut QueryContext<'a, F, M, R>,
    req: GetKeyDataReq,
) -> QueryResult {
    todo!()
}
