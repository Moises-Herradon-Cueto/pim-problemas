use material_yew::text_inputs::MatTextField;
use material_yew::text_inputs::TextFieldType;
use yew::{prelude::*, virtual_dom::AttrValue};

pub struct Comp;

pub enum Msg {
    Start(String),
    End(String),
}

pub enum Which {
    Start,
    End,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub cb: Callback<(usize, Which)>,
    pub start: usize,
    pub end: usize,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (which, number) = match msg {
            Msg::Start(x) => (Which::Start, x),
            Msg::End(x) => (Which::End, x),
        };

        let number: Result<usize, _> = number.parse();
        let Ok(mut number) = number else {return false};
        if matches!(which, Which::Start) {
            number = number.saturating_sub(1);
        }
        ctx.props().cb.emit((number, which));

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let start = (ctx.props().start + 1).to_string();
        let end = ctx.props().end.to_string();
        let start_input = ctx.link().callback(Msg::Start);
        let end_input = ctx.link().callback(Msg::End);
        html! {
                    <p>{"Mostrar los resultados del "}
                    <MatTextField
                    label={Some(AttrValue::Static("Inicio"))}
                    outlined={true}
                    field_type={TextFieldType::Number}
                    value={Some(AttrValue::Owned(start))}
                    oninput={start_input}
                        />
                    {" al "}
                    <MatTextField
                    label={Some(AttrValue::Static("Inicio"))}
                    outlined={true}
                    field_type={TextFieldType::Number}
                    value={Some(AttrValue::Owned(end))}
                    oninput={end_input}
                        />
        </p>
                }
    }
}
