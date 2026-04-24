use std::{error::Error, process, time::Duration};

use upm_common::{Resp, req::BlinkReq, resp::BlinkEndedResp};

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
    let mut io = Io::new().await.unwrap();

    tokio::time::sleep(Duration::from_secs(1)).await;

    io.send(BlinkReq { n: 3 }).await?;

    match io.listen().await? {
        Resp::BlinkEnded(BlinkEndedResp { n }) => {
            println!("blinked {n} times");
        }
    }

    Ok(())
}
