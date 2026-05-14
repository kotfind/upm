use dialoguer::{Input, Password, theme::ColorfulTheme};
use heapless::String;
use tokio::io::AsyncWriteExt;
use upm_common::{
    Resp,
    model::KeyKind,
    req::{GetKeyDataReq, GetKeyMetaReq},
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
        .with_prompt("Name")
        .interact_text()?;

    ctx.io.send(GetKeyMetaReq { name: name.clone() }).await?;

    let meta = match ctx.io.listen().await? {
        Resp::GotKeyMeta(meta) => meta,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    let passwd = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Password (hint: {})", meta.passwd_hint))
        .interact()?;

    ctx.io
        .send(GetKeyDataReq {
            name,
            passwd: passwd.to_heapless_string()?,
        })
        .await?;

    let data = match ctx.io.listen().await? {
        Resp::GotKeyData(data) => data,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    match data.kind {
        KeyKind::Bytes(bytes) => tokio::io::stdout().write_all(&bytes).await?,
        KeyKind::ChaCha20Poly1305(key) => {
            let h = hex::encode(key);
            println!("{h}");
        }
        KeyKind::K256(key) => {
            let bytes = key.to_bytes();
            let h = hex::encode(bytes);
            println!("{h}");
        }
    }

    Ok(())
}
