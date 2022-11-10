use std::{collections::HashMap, path::PathBuf, rc::Rc};

use parse_lib::{Data, ParseOneError};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::{
    app::invoke,
    files_info::{Paths, DEFAULT_DB, DEFAULT_PROBLEMS},
};

#[derive(Default)]
pub struct UpdateDb {
    output: Vec<ParseOneError>,
    error: String,
}

pub enum Msg {
    ParseFiles,
    UpdateOutput(Vec<ParseOneError>),
    UpdateErr(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub paths: Paths,
    pub cb: Callback<Rc<HashMap<usize, Data>>>,
    pub db: Option<Rc<HashMap<usize, Data>>>,
}

impl Component for UpdateDb {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ParseFiles => {
                log::info!("Parsing");
                let paths = ctx.props().paths.clone();
                let db = ctx.props().db.clone().unwrap_or_default();

                ctx.link().send_future(async move {
                    let parsed = Self::parse_files(paths, &db).await;
                    log::info!("Parsed: {parsed:#?}");
                    match parsed {
                        Ok(errors) => Msg::UpdateOutput(errors),
                        Err(err) => Msg::UpdateErr(err),
                    }
                });
                self.error = String::from("Cargando...");
                true
            }
            Msg::UpdateErr(error) => {
                self.error = error;
                true
            }
            Msg::UpdateOutput(output) => {
                self.output = output;
                self.error = String::new();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_: MouseEvent| Msg::ParseFiles);
        html! {
            <div>
            <button onclick={onclick}>{"Actualizar"}</button>
            <p>{&self.error}</p>
            <ul>
            {self.output.iter().map(show_error).collect::<Html>()}
            </ul>

            </div>
        }
    }
}

fn show_error(error: &ParseOneError) -> Html {
    if matches!(error, ParseOneError::NotTex(_)) {
        html! {}
    } else {
        html!(
            <li>
            {error.to_string()}
            </li>
        )
    }
}

#[derive(Serialize, Deserialize)]
struct UpdateArgs {
    #[serde(rename = "problemsPath")]
    problems_path: PathBuf,
    #[serde(rename = "dbPath")]
    db_path: PathBuf,
    db: String,
}
impl UpdateDb {
    #[allow(clippy::future_not_send)]
    async fn parse_files(
        paths: Paths,
        db: &HashMap<usize, Data>,
    ) -> Result<Vec<ParseOneError>, String> {
        let db = serde_json::to_string(&db)
            .map_err(|err| format!("Failed to serialize arguments: {err}"))?;
        let problems_path = paths
            .problems
            .unwrap_or_else(|| PathBuf::from(DEFAULT_PROBLEMS));
        let db_path = paths.database.unwrap_or_else(|| PathBuf::from(DEFAULT_DB));
        let args = serde_wasm_bindgen::to_value(&UpdateArgs {
            problems_path,
            db_path,
            db,
        })
        .expect("Couldn't make into js value🫣");
        let invoke_result = invoke("update_db", args).await;
        let parsed_result: Result<String, String> =
            serde_wasm_bindgen::from_value(invoke_result)
                .map_err(|err| format!("Error converting js value to value: {err}"))?;
        let value = parsed_result?;
        log::info!("About to deserialize: {value:#}");
        serde_json::from_str(&value).map_err(|err| format!("Error deserializing {err}"))
    }
}
