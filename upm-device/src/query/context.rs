use ekv::flash::Flash;
use embassy_sync::blocking_mutex::raw::RawMutex;
use rekv::Db;

use crate::io::Io;

pub struct QueryContext<'a, F: Flash, M: RawMutex> {
    pub db: Db<F, M>,
    pub io: Io<'a>,
}
