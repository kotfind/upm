use clap::Subcommand;

use crate::cmd::{CmdContext, encode_data, gen_key, read_key, write_key};

#[derive(Subcommand, Clone)]
// the variant names are turned into cli command names
#[allow(clippy::enum_variant_names)]
pub enum Cmd {
    WriteKey,
    ReadKey,
    GenKey,
    EncodeData,
}

pub async fn send(ctx: &mut CmdContext, cmd: Cmd) {
    let cmd_result = match cmd {
        Cmd::WriteKey => write_key::process(ctx).await,
        Cmd::ReadKey => read_key::process(ctx).await,
        Cmd::GenKey => gen_key::process(ctx).await,
        Cmd::EncodeData => encode_data::process(ctx).await,
    };

    if let Err(e) = cmd_result {
        println!("failed to execute a command: {e}")
    }
}
