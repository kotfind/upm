use dialoguer::{Input, Password, theme::ColorfulTheme};
use upm_common::{Resp, model::KeyKind, req::WriteKeyReq, resp::WroteKeyResp};

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

    let key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Key")
        .interact()?;

    let key_confirm = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Confirm Key")
        .interact()?;

    if key_confirm != key {
        return Err(CmdError::Confirm {
            field: "key".to_owned(),
        });
    }

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

    let passwd_hint: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Password Hint")
        .interact_text()?;

    ctx.io
        .send(WriteKeyReq {
            name,
            passwd_hint: passwd_hint.to_heapless_string(),
            passwd: passwd.to_heapless_string(),
            kind: KeyKind::Text(key.to_heapless_string()),
        })
        .await?;

    match ctx.io.listen().await? {
        Resp::WroteKey(WroteKeyResp { id }) => {
            println!("Device wrote record with id={}", id);
        }
        Resp::Error(e) => Err(e)?,
        _ => Err(CmdError::UnexpectedResponse)?,
    }

    Ok(())
}
