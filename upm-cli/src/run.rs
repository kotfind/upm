use clap::Parser;
use thiserror::Error;

use crate::{
    cmd::{self, Cmd, CmdContext},
    io::{self, Io},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
}

/// A command line interface for USB Password Manager.
#[derive(Parser)]
#[command(author, version, about, disable_help_subcommand = true)]
struct Args {
    /// A subcommand to execute.
    #[command(subcommand)]
    cmd: Cmd,
}

pub async fn run() -> Result<(), Error> {
    let args = Args::parse();

    let io = Io::new().await?;
    let mut ctx = CmdContext { io };

    cmd::send(&mut ctx, args.cmd).await;

    Ok(())
}
