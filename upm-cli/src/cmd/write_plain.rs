use dialoguer::{Input, theme::ColorfulTheme};
use upm_common::{Resp, req::WritePlainReq, resp::WrotePlainResp};

use crate::cmd::CmdContext;

pub(super) async fn process(ctx: &mut CmdContext) {
    let name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name")
        .interact_text()
        .unwrap();

    let data: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Data")
        .interact_text()
        .unwrap();

    ctx.io
        .send(WritePlainReq {
            name,
            data: data.bytes().collect(),
        })
        .await
        .unwrap();

    match ctx.io.listen().await.unwrap() {
        Resp::WrotePlain(WrotePlainResp { id }) => {
            println!("Device wrote record with id={}", id);
        }
        Resp::Error(_) => todo!(),
        _ => {
            panic!("got unexpected response")
        }
    }
}
