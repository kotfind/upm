use core::error::Error;

use log::error;

pub fn log_error(mut e: &dyn Error) {
    error!("Error occcured");

    loop {
        error!("- {e}");

        match e.source() {
            Some(next_e) => e = next_e,
            None => break,
        }
    }
}
