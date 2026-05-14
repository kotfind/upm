use chacha20poly1305::{Tag, XNonce};
use dialoguer::{Input, Password, theme::ColorfulTheme};
use file_picker::{Picker, PickerBuilder};
use heapless::String;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use upm_common::{
    Resp,
    req::{DecodeDataReq, GetKeyMetaReq},
};

use crate::{
    cmd::{
        CmdContext,
        error::{CmdError, CmdResult},
    },
    util::ToHeaplessString,
};

pub(super) async fn process(ctx: &mut CmdContext) -> CmdResult {
    let name: String<64> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Key Name")
        .interact_text()?;

    ctx.io.send(GetKeyMetaReq { name: name.clone() }).await?;

    let meta = match ctx.io.listen().await? {
        Resp::GotKeyMeta(meta) => meta,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    let input_file_path = Picker::file().with_prompt("Input File").select()?;
    let mut input_file = tokio::fs::File::open(input_file_path).await?;

    let output_file_dir = Picker::directory()
        .with_prompt("Output File Dir")
        .select()?;

    let output_file_name: std::string::String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Output File Name")
        .with_initial_text(output_file_dir.to_string_lossy())
        .interact_text()?;

    let output_file_path = output_file_dir.join(output_file_name);

    let mut output_file = tokio::fs::File::create_new(output_file_path).await?;

    let mut nonce = XNonce::default();
    input_file.read_exact(&mut nonce).await?;

    let mut auth_tag = Tag::default();
    input_file.read_exact(&mut auth_tag).await?;

    let mut data = std::vec::Vec::new();
    input_file.read_to_end(&mut data).await?;
    let Ok(data) = data.as_slice().try_into() else {
        return Err(CmdError::InputTooBig);
    };

    let passwd = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Password (hint: {})", meta.passwd_hint))
        .interact()?;

    ctx.io
        .send(DecodeDataReq {
            name,
            passwd: passwd.to_heapless_string()?,
            nonce,
            auth_tag,
            data,
        })
        .await?;

    let output_data = match ctx.io.listen().await? {
        Resp::DecodedData(output_data) => output_data,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    output_file.write_all(&output_data.data).await?;
    output_file.sync_all().await?;

    Ok(())
}
