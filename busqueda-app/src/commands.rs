use pim_lib::Data;
use web_sys::FormData;

use crate::requests::{MyRequest, MyResponse};

#[allow(clippy::future_not_send)]
pub async fn insert_db_info(data: Data) -> Result<(), String> {
    let response = MyRequest::post("/PIM/wp-admin/adim-ajax.php?action=problemas_edit")
        .json(&data)
        .send_no_parse()
        .await;
    match response {
        MyResponse::Ok { response: _ } => Ok(()),
        MyResponse::Code401 => Err("No estás autorizado/a".into()),
        MyResponse::Code500(x) | MyResponse::Error(x) => Err(x),
    }
}

#[allow(clippy::future_not_send)]
pub async fn delete(id: usize) -> Result<(), String> {
    let form = FormData::new().map_err(|err| format!("Failed to create form:\n{err:?}"))?;
    form.set_with_str("id", &id.to_string())
        .map_err(|err| format!("Failed to set id in request:\n{err:?}"))?;

    let response = MyRequest::post("/PIM/wp-admin/adim-ajax.php?action=problemas_delete")
        .body(&form)
        .send_no_parse()
        .await;
    match response {
        MyResponse::Ok { response: _ } => Ok(()),
        MyResponse::Code401 => Err("No estás autorizado/a".into()),
        MyResponse::Code500(x) | MyResponse::Error(x) => Err(x),
    }
}
