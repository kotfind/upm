use clap::Subcommand;

use crate::cmd::{CmdContext, write_plain};

#[derive(Subcommand, Clone)]
pub enum Cmd {
    WritePlain,
}

pub async fn send(ctx: &mut CmdContext, cmd: Cmd) {
    match cmd {
        Cmd::WritePlain => write_plain::process(ctx).await,
    }
}
