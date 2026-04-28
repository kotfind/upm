use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use rand::CryptoRng;
use rekv::Db;

use crate::io::Io;

pub struct QueryContext<'a, F: Flash, M: RawMutex, R: CryptoRng> {
    pub db: Db<F, M>,
    pub io: Io<'a>,
    pub rng: &'a mut R,
}
