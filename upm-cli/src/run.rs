use clap::Parser;
use thiserror::Error;

use crate::{
    cmd::{self, Cmd, CmdContext},
    io::{self, Io},
};

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

pub async fn run() -> Result<(), Error> {
    let args = Args::try_parse()?;

    let io = Io::new().await?;
    let mut ctx = CmdContext { io };

    cmd::send(&mut ctx, args.cmd).await;

    Ok(())
}
