use dialoguer::{Input, Password, Select, theme::ColorfulTheme};
use upm_common::{Resp, model::KeyTy, req::GenKeyReq, resp::GenedKeyResp};

use crate::{
    cmd::{
        CmdContext,
        error::{CmdError, CmdResult},
    },
    util::ToHeaplessString,
};

pub(super) async fn process(ctx: &mut CmdContext) -> CmdResult {
    let name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name")
        .interact_text()?;

    let ty_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key Type")
        .items(KeyTy::ALL)
        .interact()?;
    let ty = KeyTy::ALL[ty_idx];

    let passwd = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()?;

    let passwd_confirm = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Confirm Password")
        .interact()?;

    if passwd_confirm != passwd {
        return Err(CmdError::Confirm {
            field: "password".to_owned(),
        });
    }

    let passwd_hint = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Password Hint")
        .interact_text()?;

    ctx.io
        .send(GenKeyReq {
            name,
            passwd_hint,
            passwd: passwd.to_heapless_string()?,
            ty,
        })
        .await?;

    match ctx.io.listen().await? {
        Resp::GenedKey(GenedKeyResp { id }) => {
            println!("Device generated a record with id={}", id);
        }
        Resp::Error(e) => Err(e)?,
        _ => Err(CmdError::UnexpectedResponse)?,
    }

    Ok(())
}
