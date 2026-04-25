use std::process;

use crate::util::print_error;

mod io;
mod run;
mod util;

#[tokio::main]
async fn main() {
    if let Err(e) = run::run().await {
        print_error(&e);
        process::exit(1);
    }
}
