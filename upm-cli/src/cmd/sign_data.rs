use dialoguer::{Input, Password, theme::ColorfulTheme};
use file_picker::{Picker, PickerBuilder};
use heapless::String;
use upm_common::{
    Resp,
    req::{GetKeyMetaReq, SignDataReq},
    resp::SignedDataResp,
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

    let input_data = tokio::fs::read(input_file_path).await?;
    let Ok(input_data) = input_data.as_slice().try_into() else {
        return Err(CmdError::InputTooBig);
    };

    let passwd = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Password (hint: {})", meta.passwd_hint))
        .interact()?;

    ctx.io
        .send(SignDataReq {
            name,
            passwd: passwd.to_heapless_string()?,
            data: input_data,
        })
        .await?;
    let SignedDataResp { sgn } = match ctx.io.listen().await? {
        Resp::SignedData(res) => res,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    let h = hex::encode(sgn.to_bytes());
    println!("{h}");

    Ok(())
}
