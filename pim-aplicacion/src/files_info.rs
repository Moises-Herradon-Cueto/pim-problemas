use std::{borrow::Cow, fmt::Display, path::PathBuf, str::FromStr};
use yew::prelude::*;

use crate::{app::invoke, helper::GetFolderArgs};

pub const DEFAULT_PROBLEMS: &str = if cfg!(debug_assertions) {
    "/home/moises/OneDrive/ejercicios"
} else {
    ""
};

pub const DEFAULT_DB: &str = if cfg!(debug_assertions) {
    "/home/moises/OneDrive/ejercicios/database.json"
} else {
    "."
};

pub const DEFAULT_OUTPUT: &str = if cfg!(debug_assertions) {
    "/home/moises/pim-input/ejercicios-out"
} else {
    "."
};

#[derive(Clone, Copy)]
pub enum PathTo {
    Problems,
    Output,
}

#[derive(Default)]
pub struct Comp {
    problems_directory: Option<PathBuf>,
    database_directory: Option<PathBuf>,
    output_directory: Option<PathBuf>,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Paths {
    pub problems: Option<PathBuf>,
    pub database: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

pub enum MsgUpdate {
    Problems(PathBuf),
    Output(PathBuf),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub paths: Paths,
    pub update_cb: Callback<Paths>,
}

impl Component for Comp {
    type Message = MsgUpdate;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            problems_directory: ctx.props().paths.problems.clone(),
            database_directory: ctx.props().paths.database.clone(),
            output_directory: ctx.props().paths.output.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgUpdate::Problems(s) => {
                self.problems_directory = Some(s);
            }
            MsgUpdate::Output(s) => {
                self.output_directory = Some(s.clone());
                self.database_directory = Some(s.join("database.json"));
            }
        }
        ctx.props().update_cb.emit(Paths {
            problems: self.problems_directory.clone(),
            database: self.database_directory.clone(),
            output: self.output_directory.clone(),
        });
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="file-info-container">
            {self.file_input(PathTo::Problems, ctx)}
            {self.file_input(PathTo::Output, ctx)}
            </div>
        }
    }
}

pub fn _default_problem_dir() -> PathBuf {
    PathBuf::from_str(DEFAULT_PROBLEMS).unwrap()
}

pub fn _default_db_dir() -> PathBuf {
    PathBuf::from_str(DEFAULT_DB).unwrap()
}

impl Comp {
    fn file_input(&self, path_to: PathTo, ctx: &Context<Self>) -> Html {
        let current_path = self
            .get_path(path_to)
            .as_ref()
            .map_or_else(|| Cow::Owned(path_to.default_path()), Cow::Borrowed);

        let link = ctx.link().clone();
        let onclick = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            log::info!("CLICK!");
            link.send_future_batch(async move {
                let result = invoke(
                    "get_folder",
                    serde_wasm_bindgen::to_value(&GetFolderArgs).unwrap(),
                )
                .await;

                log::info!("{result:?}");

                let result: Option<PathBuf> = serde_wasm_bindgen::from_value(result).unwrap();

                let Some(result) = result else {return vec![];};

                match path_to {
                    PathTo::Problems => vec![MsgUpdate::Problems(result)],
                    PathTo::Output => vec![MsgUpdate::Output(result)],
                }
            });
        });
        html! {
            <span  class="file-info" >{format!("{path_to}: ")}<span {onclick} title="Cambiar" class="file-info-link">{current_path.display()}</span></span>
        }
    }

    const fn get_path(&self, path_to: PathTo) -> &Option<PathBuf> {
        match path_to {
            PathTo::Problems => &self.problems_directory,
            PathTo::Output => &self.output_directory,
        }
    }
}

impl PathTo {
    fn default_path(self) -> PathBuf {
        match self {
            Self::Problems => PathBuf::from(DEFAULT_PROBLEMS),
            Self::Output => PathBuf::from(DEFAULT_OUTPUT),
        }
    }
}

impl Display for PathTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Problems => write!(f, "Problemas"),
            Self::Output => write!(f, "Carpeta vacía para escribir nuevos .tex"),
        }
    }
}
