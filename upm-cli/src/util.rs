use std::error::Error;

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
    fn to_heapless_string<const CAP: usize>(&self) -> heapless::String<CAP>;
}

impl<T: AsRef<str>> ToHeaplessString for T {
    fn to_heapless_string<const CAP: usize>(&self) -> heapless::String<CAP> {
        self.as_ref().try_into().unwrap()
    }
}
