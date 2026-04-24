use std::{error::Error, process, time::Duration};

use upm_common::msg::Msg;

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

    io.write_cbor(&Msg {
        text: "A!".try_into().unwrap(),
    })
    .await?;

    let msg = io.read_cbor::<Msg>().await?;
    println!("{}", msg.text);

    io.write_cbor(&Msg {
        text: "B!".try_into().unwrap(),
    })
    .await?;

    let msg = io.read_cbor::<Msg>().await?;
    println!("{}", msg.text);

    io.write_cbor(&Msg {
        text: "C!".try_into().unwrap(),
    })
    .await?;

    let msg = io.read_cbor::<Msg>().await?;
    println!("{}", msg.text);

    Ok(())
}
