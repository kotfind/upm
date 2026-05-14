use std::{fs, io::Read, path::PathBuf};

use dialoguer::{Input, theme::ColorfulTheme};
use file_picker::{Picker, PickerBuilder};
use heapless::String;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use upm_common::{
    Req, Resp,
    req::{EncodeDataReq, GetKeyMetaReq},
    resp::EncodedDataResp,
};

use crate::cmd::{
    CmdContext,
    error::{CmdError, CmdResult},
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

    let output_file_dir = Picker::directory()
        .with_prompt("Output File Dir")
        .select()?;

    let output_file_name: std::string::String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Output File Name")
        .with_initial_text(output_file_dir.to_string_lossy())
        .interact_text()?;

    let output_file_path = output_file_dir.join(output_file_name);

    let mut output_file = tokio::fs::File::create_new(output_file_path).await?;

    let input_data = tokio::fs::read(input_file_path).await?;
    let Ok(input_data) = input_data.as_slice().try_into() else {
        return Err(CmdError::InputTooBig);
    };

    let passwd = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Password (hint: {})", meta.passwd_hint))
        .interact_text()?;

    ctx.io
        .send(EncodeDataReq {
            name,
            passwd,
            data: input_data,
        })
        .await?;
    let EncodedDataResp {
        nonce,
        auth_tag,
        data,
    } = match ctx.io.listen().await? {
        Resp::EncodedData(res) => res,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    output_file.write_all(&nonce).await?;
    output_file.write_all(&auth_tag).await?;
    output_file.write_all(&data).await?;

    output_file.sync_all().await?;

    Ok(())
}
