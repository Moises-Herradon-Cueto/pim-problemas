use crate::RawHtml;
use material_yew::select::{ListIndex, MatSelect, SelectedDetail};
use material_yew::text_inputs::MatTextArea;
use material_yew::text_inputs::MatTextField;
use material_yew::text_inputs::TextFieldType;
use material_yew::MatListItem;
use pim_lib::{FieldContents, Fields, CURSOS};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use FieldContents::{
    Author, Comments, Difficulty, Figures, History, Id, Packages, PdfUrl, Problem, Source, TexUrl,
    Title, Topics, Year,
};

pub struct Comp {
    contents: FieldContents,
    error: Option<String>,
    showing_problem: bool,
}

pub enum Msg {
    Edit(String),
    ShowPreview,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub edit_cb: Callback<FieldContents>,
    pub contents: FieldContents,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            contents: ctx.props().contents.clone(),
            error: None,
            showing_problem: matches!(ctx.props().contents, Problem(_)),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Edit(content) => {
                let parsed = Fields::from(&self.contents).parse(&content);
                match parsed {
                    Err(err) => {
                        self.error = Some(err);
                    }
                    Ok(content) => {
                        self.contents = content;
                        self.error = None;
                        self.showing_problem = false;
                        ctx.props().edit_cb.emit(self.contents.clone());
                    }
                }

                true
            }
            Msg::ShowPreview => {
                self.showing_problem = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !Fields::from(&ctx.props().contents).editable() {
            return html! {
                <div class="entry-uneditable">{&ctx.props().contents}</div>
            };
        }

        let oninput = ctx.link().callback(Msg::Edit);
        if let Id(id) = self.contents {
            return html! {<h3>{format!("Editando el problema {id}")}</h3>};
        }
        let field_type = match &self.contents {
            Id(_) | Difficulty(_) => TextFieldType::Number,
            _ => TextFieldType::Text,
        };

        let problem = matches!(self.contents, Problem(_));

        let input_type = match &self.contents {
            Id(_) | Title(_) | Difficulty(_) | Source(_) | Author(_) | TexUrl(_) | PdfUrl(_) => {
                InputType::One
            }
            Figures(_) | Problem(_) | History(_) | Comments(_) | Topics(_) | Packages(_) => {
                InputType::Multi
            }
            Year(_) => InputType::Year,
        };

        if !self.showing_problem {
            ctx.link().send_message(Msg::ShowPreview);
        }

        string_input(
            &self.contents,
            oninput,
            field_type,
            problem,
            self.showing_problem && problem,
            input_type,
            &self.error,
        )
    }
}

#[derive(Clone, Copy)]
enum InputType {
    One,
    Multi,
    Year,
}

fn string_input(
    contents: &FieldContents,
    oninput: Callback<String>,
    field_type: TextFieldType,
    problem: bool,
    show_output: bool,
    input_type: InputType,
    error: &Option<String>,
) -> Html {
    let field = Fields::from(contents);
    let output = if show_output {
        html! {<RawHtml id={AttrValue::from("problem-preview")} inner_html={AttrValue::from(contents.string_contents().to_string())} tag={"p"}/>}
    } else {
        html! {}
    };

    let helper = error
        .as_ref()
        .map_or(AttrValue::Static(""), |x| AttrValue::Owned(x.clone()));

    let rows = if problem { 10 } else { 2 };

    let input_mat = match input_type {
        InputType::One => html! {
        <MatTextField
            label={Some(AttrValue::Owned(field.to_string()))}
            outlined={true}
            {field_type}
            value={Some(AttrValue::Owned(contents.string_contents().into_owned()))}
            {oninput}
            {helper}
            />
        },
        InputType::Multi => html! {
        <MatTextArea
            label={Some(AttrValue::Owned(field.to_string()))}
            outlined={true}
            {field_type}
            value={Some(AttrValue::Owned(contents.string_contents().into_owned()))}
            {oninput}
            {helper}
            {rows}
            cols={Some(200)}
            />
        },
        InputType::Year => {
            let onselected = oninput.reform(|x: SelectedDetail| {
                let selection = x.index;
                match selection {
                    ListIndex::Single(Some(0) | None) => String::new(),
                    ListIndex::Single(Some(i)) => CURSOS[i - 1].to_string(),
                    ListIndex::Multi(_) => {
                        log::warn!("{selection:?}");
                        String::new()
                    }
                }
            });

            let cursos: Html = CURSOS
                .into_iter()
                .map(|curso| {
                    let selected = if let FieldContents::Year(Some(x)) = contents {
                        curso == *x
                    } else {
                        false
                    };
                    html! {
                        <MatListItem {selected}>{curso}</MatListItem>
                    }
                })
                .collect();

            html! {
                          <MatSelect {onselected}
            outlined={true}>
                            <MatListItem>{"Elige el curso"}</MatListItem>
                            {cursos}
                        </MatSelect>
            }
        }
    };

    html! {
        <div>
        {input_mat}
        {output}
        </div>
    }
}
