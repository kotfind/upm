use clap::Subcommand;

use crate::cmd::{CmdContext, write_key};

#[derive(Subcommand, Clone)]
pub enum Cmd {
    WriteKey,
}

pub async fn send(ctx: &mut CmdContext, cmd: Cmd) {
    match cmd {
        Cmd::WriteKey => write_key::process(ctx).await,
    }
}
