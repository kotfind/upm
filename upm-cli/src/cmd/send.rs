use clap::Subcommand;

use crate::cmd::{CmdContext, read_key, write_key};

#[derive(Subcommand, Clone)]
pub enum Cmd {
    WriteKey,
    ReadKey,
}

pub async fn send(ctx: &mut CmdContext, cmd: Cmd) {
    let cmd_result = match cmd {
        Cmd::WriteKey => write_key::process(ctx).await,
        Cmd::ReadKey => read_key::process(ctx).await,
    };

    if let Err(e) = cmd_result {
        println!("failed to execute a command: {e}")
    }
}
