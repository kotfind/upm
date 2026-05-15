use upm_common::{Resp, req::ListKeysReq, resp::ListedKeyResp};

use crate::cmd::{
    CmdContext,
    error::{CmdError, CmdResult},
};

pub(super) async fn process(ctx: &mut CmdContext) -> CmdResult {
    ctx.io.send(ListKeysReq).await?;

    let mut key_names = Vec::new();
    loop {
        match ctx.io.listen().await? {
            Resp::ListedKey(ListedKeyResp::Key { name }) => key_names.push(name),
            Resp::ListedKey(ListedKeyResp::EndOfList) => break,
            Resp::Error(e) => return Err(e.into()),
            _ => return Err(CmdError::UnexpectedResponse),
        }
    }

    if key_names.is_empty() {
        eprintln!("no keys found");
    } else {
        eprintln!("{} keys found:", key_names.len());
        for key_name in key_names {
            println!("{key_name}");
        }
    }

    Ok(())
}
