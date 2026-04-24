use std::{error::Error, process, time::Duration};

use crate::io::Io;

mod io;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => (),
        Err(e) => {
            let mut e: &dyn Error = &e;

            loop {
                eprintln!("- {e}");

                if let Some(next_e) = e.source() {
                    e = next_e;
                } else {
                    process::exit(1);
                }
            }
        }
    }
}

async fn run() -> Result<(), io::Error> {
    let mut io = Io::new().await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    loop {
        io.write_bytes(b"Hello, world!").await?;
        let ans = io.read_bytes().await?;
        println!("-- {}", str::from_utf8(&ans).unwrap());
    }
}
