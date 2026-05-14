use std::error::Error;

use crate::cmd::CmdError;

pub fn print_error(mut e: &dyn Error) {
    eprintln!("Error occured:");

    loop {
        eprintln!("- {e}");

        if let Some(next_e) = e.source() {
            e = next_e;
        } else {
            break;
        }
    }
}

pub trait ToHeaplessString {
    fn to_heapless_string<const CAP: usize>(&self) -> Result<heapless::String<CAP>, CmdError>;
}

impl<T: AsRef<str>> ToHeaplessString for T {
    fn to_heapless_string<const CAP: usize>(&self) -> Result<heapless::String<CAP>, CmdError> {
        self.as_ref().try_into().map_err(|_| CmdError::InputTooBig)
    }
}
