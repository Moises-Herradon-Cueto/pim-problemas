use std::{path::PathBuf, str::FromStr};

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[cfg(debug_assertions)]
pub const DEFAULT_PROBLEMS: &str = if cfg!(debug_assertions) {
    "/home/moises/problems_in/"
} else {
    "."
};

#[cfg(debug_assertions)]
pub const DEFAULT_DB: &str = if cfg!(debug_assertions) {
    "/home/moises/pim-input/database.json"
} else if cfg!(target = "windows") {
    ".\\base_de_datos.json"
} else {
    "./base_de_datos.json"
};

#[derive(Default)]
pub struct Comp {
    problems_directory: Option<PathBuf>,
    database_directory: Option<PathBuf>,
    problems_ref: NodeRef,
    database_ref: NodeRef,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Paths {
    pub problems: Option<PathBuf>,
    pub database: Option<PathBuf>,
}

pub enum Msg {
    UpdateProblems(String),
    UpdateDb(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub paths: Paths,
    pub update_cb: Callback<Paths>,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            problems_directory: ctx.props().paths.problems.clone(),
            database_directory: ctx.props().paths.database.clone(),
            ..Self::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateProblems(s) => {
                self.problems_directory = Some(PathBuf::from_str(&s).unwrap());
                ctx.props().update_cb.emit(Paths {
                    problems: self.problems_directory.clone(),
                    database: self.database_directory.clone(),
                });
                false
            }
            Msg::UpdateDb(s) => {
                self.database_directory = Some(PathBuf::from_str(&s).unwrap());
                ctx.props().update_cb.emit(Paths {
                    problems: self.problems_directory.clone(),
                    database: self.database_directory.clone(),
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let problems_ref_2 = self.problems_ref.clone();
        let input_problem = ctx.link().callback(move |_: InputEvent| {
            let data = get_value_from_ref(&problems_ref_2);
            Msg::UpdateProblems(data)
        });

        let problem = if self.problems_directory.is_none() {
            html! {
                <input name="problems_directory" oninput={input_problem} ref={self.problems_ref.clone()} value={DEFAULT_PROBLEMS} />
            }
        } else {
            html! {
                <input name="problems_directory" oninput={input_problem} ref={self.problems_ref.clone()} />
            }
        };

        let database_ref_2 = self.database_ref.clone();
        let input_database = ctx.link().callback(move |_: InputEvent| {
            let data = get_value_from_ref(&database_ref_2);
            Msg::UpdateDb(data)
        });
        let database = if self.database_directory.is_none() {
            html! {
                <input name="database" oninput={input_database} ref={self.database_ref.clone()} value={DEFAULT_DB} />
            }
        } else {
            html! {
                <input name="database" oninput={input_database} ref={self.database_ref.clone()} />
            }
        };
        html! {
            <form class="file_info">
                <label for="problems_directory">{"Carpeta con los problemas"}</label>
                {problem}
                <label for="database">{"Base de datos"}</label>
                {database}
            </form>
        }
    }
}

fn get_value_from_ref(elt: &NodeRef) -> String {
    elt.cast::<HtmlInputElement>().map_or_else(
        || String::from("Had a big problem, since this is not an input element"),
        |elt| elt.value(),
    )
}

pub fn _default_problem_dir() -> PathBuf {
    PathBuf::from_str(DEFAULT_PROBLEMS).unwrap()
}

pub fn _default_db_dir() -> PathBuf {
    PathBuf::from_str(DEFAULT_DB).unwrap()
}
