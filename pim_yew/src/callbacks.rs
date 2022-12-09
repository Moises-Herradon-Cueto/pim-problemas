#[macro_export]
macro_rules! button_callback {
    ($ctx:ident, $msg:expr,$info:expr) => {{
        let info = $info;
        $ctx.link().callback(move |e: MouseEvent| {
            e.prevent_default();
            $msg(info)
        })
    }};
}

pub(crate) use button_callback;

// pub fn button_callback<Comp, Msg>(ctx: &Context<Comp>, out: Msg) -> Callback<MouseEvent>
// where
//     Comp: Component<Message = Msg>,
// {
//     ctx.link().callback(move |e: MouseEvent| {
//         e.prevent_default();
//         out
//     })
// }
