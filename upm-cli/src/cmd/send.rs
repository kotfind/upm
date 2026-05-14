use clap::Subcommand;

use crate::cmd::{CmdContext, decode_data, encode_data, gen_key, list_keys, read_key, write_key};

#[derive(Subcommand, Clone)]
// the variant names are turned into cli command names
#[allow(clippy::enum_variant_names)]
pub enum Cmd {
    /// Writes a new key to the device's memory.
    WriteKey,

    /// Randomly generates a key and writes it to the device's memory.
    GenKey,

    /// Reads a key's data.
    ReadKey,

    /// Encodes a file, using a symmetric key, stored in the device's memory.
    EncodeData,

    /// Decodes a file, using a symmetric key, previously encoded with the same key.
    DecodeData,

    /// List names of all the keys.
    ListKeys,
}

pub async fn send(ctx: &mut CmdContext, cmd: Cmd) {
    let cmd_result = match cmd {
        Cmd::WriteKey => write_key::process(ctx).await,
        Cmd::ReadKey => read_key::process(ctx).await,
        Cmd::GenKey => gen_key::process(ctx).await,
        Cmd::EncodeData => encode_data::process(ctx).await,
        Cmd::DecodeData => decode_data::process(ctx).await,
        Cmd::ListKeys => list_keys::process(ctx).await,
    };

    if let Err(e) = cmd_result {
        println!("failed to execute a command: {e}")
    }
}
