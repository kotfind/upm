use clap::Subcommand;

use crate::cmd::{CmdContext, write_key};

#[derive(Subcommand, Clone)]
pub enum Cmd {
    WriteKey,
}

pub async fn send(ctx: &mut CmdContext, cmd: Cmd) {
    let cmd_result = match cmd {
        Cmd::WriteKey => write_key::process(ctx).await,
    };

    if let Err(e) = cmd_result {
        println!("failed to execute a command: {e}")
    }
}
