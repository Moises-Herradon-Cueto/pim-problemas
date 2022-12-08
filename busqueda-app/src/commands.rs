use pim_lib::Data;

use crate::requests::{MyRequest, MyResponse};

#[allow(clippy::future_not_send)]
pub async fn insert_db_info(data: Data) -> Result<(), String> {
    let response = MyRequest::post("/PIM/externos/intranet/problemas-edit.php")
        .json(&data)
        .send_no_parse()
        .await;
    match response {
        MyResponse::Ok { response: _ } => Ok(()),
        MyResponse::Code401 => Err("No estás autorizado/a".into()),
        MyResponse::Code500(x) | MyResponse::Error(x) => Err(x),
    }
}
