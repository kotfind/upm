use dialoguer::{Input, theme::ColorfulTheme};
use file_picker::{Picker, PickerBuilder};
use heapless::String;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use upm_common::{
    Req, Resp,
    model::{DATA_CHUNK_SIZE, DataChunk},
    req::{EncodeDataReq, GetKeyMetaReq},
};

use crate::cmd::{
    CmdContext,
    error::{CmdError, CmdResult},
};

pub(super) async fn process(ctx: &mut CmdContext) -> CmdResult {
    let res = process_inner(ctx).await;

    if res.is_err() {
        ctx.io.send(Req::EndOfData).await?;
        let _ = ctx.io.listen().await;
    }

    res
}

async fn process_inner(ctx: &mut CmdContext) -> CmdResult {
    let name: String<64> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Key Name")
        .interact_text()?;

    ctx.io.send(GetKeyMetaReq { name: name.clone() }).await?;

    let meta = match ctx.io.listen().await? {
        Resp::GotKeyMeta(meta) => meta,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    let passwd = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Password (hint: {})", meta.passwd_hint))
        .interact_text()?;

    ctx.io.send(EncodeDataReq { name, passwd }).await?;
    match ctx.io.listen().await? {
        Resp::EncodeDataInitOk => {}
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    }

    let file_path = Picker::file().with_prompt("File").select()?;

    let mut in_file = tokio::fs::File::open(file_path).await?;

    loop {
        let mut buf = [0u8; DATA_CHUNK_SIZE];
        let data = in_file.read(&mut buf).await.map(|len| &buf[..len])?;

        if data.is_empty() {
            ctx.io.send(Req::EndOfData).await?;
            match ctx.io.listen().await? {
                Resp::EndOfData => break,
                Resp::Error(e) => return Err(e.into()),
                _ => return Err(CmdError::UnexpectedResponse),
            }
        }

        ctx.io
            .send(DataChunk {
                data: data.try_into().unwrap(),
            })
            .await?;
        match ctx.io.listen().await? {
            Resp::DataChunk(chunk) => {
                tokio::io::stdout().write_all(&chunk.data).await?;
            }
            Resp::Error(e) => return Err(e.into()),
            _ => return Err(CmdError::UnexpectedResponse),
        }
    }

    Ok(())
}
