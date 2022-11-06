use yew::prelude::*;

pub struct UpdateDb;

pub enum Msg {}

impl Component for UpdateDb {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}
