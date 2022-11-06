use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use web_sys::{HtmlInputElement, Node};
use yew::prelude::*;

#[cfg(debug_assertions)]
const DEFAULT_PROBLEMS: &str = if cfg!(debug_assertions) {
    "./input/problems_in/"
} else {
    "."
};

#[cfg(debug_assertions)]
const DEFAULT_DB: &str = if cfg!(debug_assertions) {
    "./input/database.json"
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

pub enum Msg {
    UpdateProblems(String),
    UpdateDb(String),
}

impl Component for Comp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateProblems(s) => {
                println!("Problems: {s}");
                false
            }
            Msg::UpdateDb(s) => {
                println!("Database: {s}");
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
                <input name="database_directory" oninput={input_database} ref={self.database_ref.clone()} value={DEFAULT_DB} />
            }
        } else {
            html! {
                <input name="database_directory" oninput={input_database} ref={self.database_ref.clone()} />
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
