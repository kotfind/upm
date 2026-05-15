use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use heapless::String;
use upm_common::{Resp, req::RemoveKeyReq};

use crate::cmd::{
    CmdContext,
    error::{CmdError, CmdResult},
};

pub(super) async fn process(ctx: &mut CmdContext) -> CmdResult {
    let name: String<64> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name")
        .interact_text()?;

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Are you sure you want to delete `{name}` key?"))
        .interact()?;

    if !confirm {
        return Err(CmdError::Confirm {
            field: "_".to_string(),
        });
    }

    ctx.io.send(RemoveKeyReq { name }).await?;

    let resp = match ctx.io.listen().await? {
        Resp::RemovedKey(resp) => resp,
        Resp::Error(e) => return Err(e.into()),
        _ => return Err(CmdError::UnexpectedResponse),
    };

    println!("Removed key with id={}", resp.id);

    Ok(())
}
