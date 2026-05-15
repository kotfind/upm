use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use rand::CryptoRng;
use rekv::Db;

use crate::{confirm_io::ConfirmIo, io::Io};

pub struct QueryContext<'a, F: Flash, M: RawMutex, R: CryptoRng> {
    pub db: Db<F, M>,
    pub io: Io<'a>,
    pub rng: R,
    pub cfm_io: ConfirmIo<'a>,
}
