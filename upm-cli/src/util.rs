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
