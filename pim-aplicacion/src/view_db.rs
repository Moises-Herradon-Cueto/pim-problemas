use std::{collections::HashMap, rc::Rc};

use parse_lib::data::Data;
use yew::prelude::*;

pub struct ViewDb {
    view: Vec<Data>,
}

pub enum Msg {}

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct Props {
    pub db: Rc<HashMap<usize, Data>>,
}

impl Component for ViewDb {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut view: Vec<_> = ctx
            .props()
            .db
            .iter()
            .map(|(_, problem_info)| problem_info.clone())
            .collect();
        view.sort_by(|a, b| a.id.cmp(&b.id));
        Self { view }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let filas: Html = self.view.iter().map(into_row).collect();
        html! {
            <table>
                <tr>
                    <th>{"Id"}</th>
                    <th>{"Temas"}</th>
                    <th>{"Dificultad"}</th>
                    <th>{"Fuente"}</th>
                    <th>{"Historial"}</th>
                    <th>{"Comentarios"}</th>
                    <th>{"Curso"}</th>
                    <th>{"Enunciado"}</th>
                    <th>{"Paquetes usados"}</th>
                </tr>
                {filas}
            </table>
        }
    }
}

fn into_row(data: &Data) -> Html {
    html! {
        <tr>
        {into_td(&data.id)}
        {into_td(&data.temas.join(", "))}
        {into_td(&data.dificultad)}
        {into_td(&data.fuente)}
        {into_td(&data.historial.join("\n"))}
        {into_td(&data.comentarios.join(","))}
        {into_td(&data.curso.as_ref().unwrap_or(&String::new()))}
        {into_td(&data.enunciado.chars().take(100).collect::<String>())}
        {into_td(&data.paquetes.join("\n"))}
        </tr>
    }
}
fn into_td<T: ToString>(x: &T) -> Html {
    html! {
        <td>{x.to_string()}</td>
    }
}
