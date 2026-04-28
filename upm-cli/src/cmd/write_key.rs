use dialoguer::{Input, Password, theme::ColorfulTheme};
use upm_common::{Resp, model::KeyKind, req::WriteKeyReq, resp::WroteKeyResp};

use crate::cmd::CmdContext;

pub(super) async fn process(ctx: &mut CmdContext) {
    let name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name")
        .interact_text()
        .unwrap();

    let key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Key")
        .interact()
        .unwrap();

    let key_confirm = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Confirm Key")
        .interact()
        .unwrap();

    if key_confirm != key {
        panic!("key confirmation failed")
    }

    let passwd = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
        .unwrap();

    let passwd_confirm = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Confirm Password")
        .interact()
        .unwrap();

    if passwd_confirm != passwd {
        panic!("password confirmation failed")
    }

    let passwd_hint: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Password Hint")
        .interact_text()
        .unwrap();

    ctx.io
        .send(WriteKeyReq {
            name,
            passwd_hint: passwd_hint.as_str().try_into().unwrap(),
            passwd: passwd.as_str().try_into().unwrap(),
            kind: KeyKind::Text(key.as_str().try_into().unwrap()),
        })
        .await
        .unwrap();

    match ctx.io.listen().await.unwrap() {
        Resp::WroteKey(WroteKeyResp { id }) => {
            println!("Device wrote record with id={}", id);
        }
        Resp::Error(_) => todo!(),
        _ => {
            panic!("got unexpected response")
        }
    }
}
