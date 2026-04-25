use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Input, theme::ColorfulTheme};
use thiserror::Error;
use upm_common::{Resp, req::BlinkReq, resp::BlinkEndedResp};

use crate::io::{self, Io};

#[derive(Error, Debug)]
pub enum Error {
    #[error("clap error")]
    Clap(#[from] clap::Error),

    #[error("io error")]
    Io(#[from] io::Error),

    #[error("dialoguer error")]
    Dialoguer(#[from] dialoguer::Error),

    #[error("unexpected response error")]
    UnexpectedResponse,
}

#[derive(Parser)]
#[command(author, version, about, disable_help_subcommand = true)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Clone)]
enum Cmd {
    Blink,
}

pub async fn run() -> Result<(), Error> {
    let args = Args::try_parse()?;

    let mut io = Io::new().await?;

    match args.cmd {
        Cmd::Blink => {
            let n: usize = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("How many times to blink?")
                .validate_with(|n_str: &String| -> Result<(), &str> {
                    n_str
                        .parse::<usize>()
                        .map(|_| ())
                        .map_err(|_| "Value should be a number")
                })
                .interact_text()?
                .parse()
                .unwrap();

            io.send(BlinkReq { n }).await?;
            let Resp::BlinkEnded(BlinkEndedResp { n }) = io.listen().await? else {
                return Err(Error::UnexpectedResponse);
            };
            println!("{}", style(format!("Device blinked {n} times.")).green());
        }
    }

    Ok(())
}
