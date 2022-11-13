use std::{path::PathBuf, str::FromStr};

use web_sys::HtmlInputElement;
use yew::prelude::*;

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

#[derive(Default)]
pub struct Comp {
    problems_directory: Option<PathBuf>,
    database_directory: Option<PathBuf>,
    output_directory: Option<PathBuf>,
    problems_ref: NodeRef,
    database_ref: NodeRef,
    output_ref: NodeRef,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Paths {
    pub problems: Option<PathBuf>,
    pub database: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

pub enum MsgUpdate {
    Problems(String),
    Db(String),
    Output(String),
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
            ..Self::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgUpdate::Problems(s) => {
                self.problems_directory = Some(PathBuf::from_str(&s).unwrap());
            }
            MsgUpdate::Db(s) => {
                self.database_directory = Some(PathBuf::from_str(&s).unwrap());
            }
            MsgUpdate::Output(s) => {
                self.output_directory = Some(PathBuf::from_str(&s).unwrap());
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
        let problems_ref_2 = self.problems_ref.clone();
        let input_problem = ctx.link().callback(move |_: InputEvent| {
            let data = get_value_from_ref(&problems_ref_2);
            MsgUpdate::Problems(data)
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
            MsgUpdate::Db(data)
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

        let output_ref_2 = self.output_ref.clone();
        let input_output = ctx.link().callback(move |_: InputEvent| {
            let data = get_value_from_ref(&output_ref_2);
            MsgUpdate::Output(data)
        });
        let output = if self.output_directory.is_none() {
            html! {
                <input name="output" oninput={input_output} ref={self.output_ref.clone()} value={DEFAULT_OUTPUT} />
            }
        } else {
            html! {
                <input name="output" oninput={input_output} ref={self.output_ref.clone()} />
            }
        };

        html! {
            <form class="file_info">
            <table>
                <tr>
                <td>
                <label for="problems_directory">{"Carpeta con los problemas"}</label>
                </td><td>
                {problem}
                </td>
                </tr>
                <tr>
                <td>
                <label for="database">{"Base de datos"}</label>
                </td><td>
                {database}
                </td>
                </tr>
                <tr>
                <td>
                <label for="output_directory">{"Carpeta para guardar los nuevos .tex"}</label>
                </td><td>
                {output}
                </td>
                </tr>
                </table>
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
