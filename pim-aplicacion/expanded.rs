#![feature(prelude_import)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::let_unit_value)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use crate::main_menu::MainMenu;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::spawn_local;
    use yew::prelude::*;
    #[allow(nonstandard_style)]
    #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
    ///
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue {
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __wbg_invoke_8316632da2a98094(
            cmd: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            args: <JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(cmd);
            drop(args);
            {
                ::std::rt::begin_panic(
                    "cannot call wasm-bindgen imported functions on \
                            non-wasm targets",
                )
            };
        }
        unsafe {
            let _ret = {
                let cmd = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cmd);
                let args = <JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(args);
                __wbg_invoke_8316632da2a98094(cmd, args)
            };
            wasm_bindgen_futures::JsFuture::from(
                <js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret),
            )
            .await
            .expect("unexpected exception")
        }
    }
    #[allow(nonstandard_style)]
    #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
    ///
    pub fn log(s: &str) {
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __wbg_log_cb96bb264e4327ef(
            s: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(s);
            {
                ::std::rt::begin_panic(
                    "cannot call wasm-bindgen imported functions on \
                            non-wasm targets",
                )
            };
        }
        unsafe {
            let _ret = {
                let s = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
                __wbg_log_cb96bb264e4327ef(s)
            };
            ()
        }
    }
    struct GreetArgs<'a> {
        name: &'a str,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'a> _serde::Serialize for GreetArgs<'a> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GreetArgs",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de: 'a, 'a> _serde::Deserialize<'de> for GreetArgs<'a> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de: 'a, 'a> {
                    marker: _serde::__private::PhantomData<GreetArgs<'a>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                    type Value = GreetArgs<'a>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct GreetArgs")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<&'a str>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GreetArgs with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(GreetArgs { name: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<&'a str> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<&'a str>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GreetArgs { name: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GreetArgs",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GreetArgs<'a>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(unused_parens)]
    pub struct app {
        _marker: ::std::marker::PhantomData<()>,
    }
    impl ::yew::functional::FunctionProvider for app {
        type TProps = ();
        fn run(_: &()) -> ::yew::html::Html {
            {
                let greet_input_ref = use_ref(NodeRef::default);
                let name = use_state(String::new);
                let greet_msg = use_state(String::new);
                {
                    let greet_msg = greet_msg;
                    let name = name.clone();
                    let name2 = name.clone();
                    use_effect_with_deps(
                        move |_| {
                            spawn_local(async move {
                                if name.is_empty() {
                                    return;
                                }
                                let new_msg =
                                    invoke("greet", to_value(&GreetArgs { name: &name }).unwrap())
                                        .await;
                                log(&new_msg.as_string().unwrap());
                                greet_msg.set(new_msg.as_string().unwrap());
                            });
                            || {}
                        },
                        name2,
                    );
                }
                let _greet = {
                    let name = name;
                    let greet_input_ref = greet_input_ref;
                    Callback::from(move |_: MouseEvent| {
                        name.set(
                            greet_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                        );
                    })
                };
                {
                    #[allow(clippy::useless_conversion)]
                    <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from({
                        let __yew_props = {
                            #[allow(clippy::no_effect)]
                            if false {
                                let _ = | __yew_props : < MainMenu as :: yew :: html :: Component > :: Properties | { } ;
                            }
                            < < MainMenu as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder () . build ()
                        };
                        ::yew::virtual_dom::VChild::<MainMenu>::new(
                            __yew_props,
                            <::yew::html::NodeRef as ::std::default::Default>::default(),
                            ::std::option::Option::None,
                        )
                    })
                }
            }
        }
    }
    #[allow(type_alias_bounds)]
    pub type App = ::yew::functional::FunctionComponent<app>;
}
mod files_info {
    use std::{path::PathBuf, str::FromStr};
    use web_sys::HtmlInputElement;
    use yew::prelude::*;
    #[cfg(debug_assertions)]
    pub const DEFAULT_PROBLEMS: &str = if true { "./input/problems_in/" } else { "." };
    #[cfg(debug_assertions)]
    pub const DEFAULT_DB: &str = if true {
        "./input/database.json"
    } else if false {
        ".\\base_de_datos.json"
    } else {
        "./base_de_datos.json"
    };
    pub struct Comp {
        problems_directory: Option<PathBuf>,
        database_directory: Option<PathBuf>,
        problems_ref: NodeRef,
        database_ref: NodeRef,
    }
    #[automatically_derived]
    impl ::core::default::Default for Comp {
        #[inline]
        fn default() -> Comp {
            Comp {
                problems_directory: ::core::default::Default::default(),
                database_directory: ::core::default::Default::default(),
                problems_ref: ::core::default::Default::default(),
                database_ref: ::core::default::Default::default(),
            }
        }
    }
    pub struct Paths {
        pub problems: Option<PathBuf>,
        pub database: Option<PathBuf>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Paths {
        #[inline]
        fn clone(&self) -> Paths {
            Paths {
                problems: ::core::clone::Clone::clone(&self.problems),
                database: ::core::clone::Clone::clone(&self.database),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Paths {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Paths {
        #[inline]
        fn eq(&self, other: &Paths) -> bool {
            self.problems == other.problems && self.database == other.database
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Paths {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Paths {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Option<PathBuf>>;
            let _: ::core::cmp::AssertParamIsEq<Option<PathBuf>>;
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Paths {
        #[inline]
        fn default() -> Paths {
            Paths {
                problems: ::core::default::Default::default(),
                database: ::core::default::Default::default(),
            }
        }
    }
    pub enum Msg {
        UpdateProblems(String),
        UpdateDb(String),
    }
    pub struct Props {
        pub paths: Paths,
        pub update_cb: Callback<Paths>,
    }
    struct PropsWrapper {
        paths_wrapper: ::std::option::Option<Paths>,
        update_cb_wrapper: ::std::option::Option<Callback<Paths>>,
    }
    impl ::std::default::Default for PropsWrapper {
        fn default() -> Self {
            PropsWrapper {
                paths_wrapper: ::std::option::Option::None,
                update_cb_wrapper: ::std::option::Option::None,
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropsBuilderStep_missing_required_prop_paths;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropsBuilderStep_missing_required_prop_update_cb;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropsBuilderStepPropsBuilder;
    #[doc(hidden)]
    pub trait PropsBuilderStep {}
    impl PropsBuilderStep for PropsBuilderStep_missing_required_prop_paths {}
    impl PropsBuilderStep for PropsBuilderStep_missing_required_prop_update_cb {}
    impl PropsBuilderStep for PropsBuilderStepPropsBuilder {}
    #[doc(hidden)]
    pub struct PropsBuilder<YEW_PROPS_BUILDER_STEP: PropsBuilderStep> {
        wrapped: ::std::boxed::Box<PropsWrapper>,
        _marker: ::std::marker::PhantomData<YEW_PROPS_BUILDER_STEP>,
    }
    impl PropsBuilder<PropsBuilderStep_missing_required_prop_paths> {
        #[doc(hidden)]
        pub fn paths(
            mut self,
            paths: impl ::yew::html::IntoPropValue<Paths>,
        ) -> PropsBuilder<PropsBuilderStep_missing_required_prop_update_cb> {
            self.wrapped.paths_wrapper = ::std::option::Option::Some(paths.into_prop_value());
            PropsBuilder {
                wrapped: self.wrapped,
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    impl PropsBuilder<PropsBuilderStep_missing_required_prop_update_cb> {
        #[doc(hidden)]
        pub fn update_cb(
            mut self,
            update_cb: impl ::yew::html::IntoPropValue<Callback<Paths>>,
        ) -> PropsBuilder<PropsBuilderStepPropsBuilder> {
            self.wrapped.update_cb_wrapper =
                ::std::option::Option::Some(update_cb.into_prop_value());
            PropsBuilder {
                wrapped: self.wrapped,
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    impl PropsBuilder<PropsBuilderStepPropsBuilder> {
        #[doc(hidden)]
        pub fn build(self) -> Props {
            Props {
                paths: ::std::option::Option::unwrap(self.wrapped.paths_wrapper),
                update_cb: ::std::option::Option::unwrap(self.wrapped.update_cb_wrapper),
            }
        }
    }
    impl ::yew::html::Properties for Props {
        type Builder = PropsBuilder<PropsBuilderStep_missing_required_prop_paths>;
        fn builder() -> Self::Builder {
            PropsBuilder {
                wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Props {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Props {
        #[inline]
        fn eq(&self, other: &Props) -> bool {
            self.paths == other.paths && self.update_cb == other.update_cb
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Props {
        #[inline]
        fn clone(&self) -> Props {
            Props {
                paths: ::core::clone::Clone::clone(&self.paths),
                update_cb: ::core::clone::Clone::clone(&self.update_cb),
            }
        }
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
                {
                    #[allow(clippy::useless_conversion)]
                    <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                        #[allow(clippy::redundant_clone, unused_braces)]
                        ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                            ::yew::virtual_dom::VTag::__new_input(
                                ::yew::html::IntoPropValue::<
                                    ::std::option::Option<::yew::virtual_dom::AttrValue>,
                                >::into_prop_value(DEFAULT_PROBLEMS),
                                false,
                                ::yew::html::IntoPropValue::<::yew::html::NodeRef>::into_prop_value(
                                    self.problems_ref.clone(),
                                ),
                                ::std::option::Option::None,
                                ::yew::virtual_dom::Attributes::Static(&[[
                                    "name",
                                    "problems_directory",
                                ]]),
                                ::yew::virtual_dom::listeners::Listeners::Pending(
                                    ::std::boxed::Box::new([
                                        ::yew::html::oninput::Wrapper::__macro_new(input_problem),
                                    ]),
                                ),
                            ),
                        ),
                    )
                }
            } else {
                {
                    #[allow(clippy::useless_conversion)]
                    <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                        #[allow(clippy::redundant_clone, unused_braces)]
                        ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                            ::yew::virtual_dom::VTag::__new_input(
                                ::std::option::Option::None,
                                false,
                                ::yew::html::IntoPropValue::<::yew::html::NodeRef>::into_prop_value(
                                    self.problems_ref.clone(),
                                ),
                                ::std::option::Option::None,
                                ::yew::virtual_dom::Attributes::Static(&[[
                                    "name",
                                    "problems_directory",
                                ]]),
                                ::yew::virtual_dom::listeners::Listeners::Pending(
                                    ::std::boxed::Box::new([
                                        ::yew::html::oninput::Wrapper::__macro_new(input_problem),
                                    ]),
                                ),
                            ),
                        ),
                    )
                }
            };
            let database_ref_2 = self.database_ref.clone();
            let input_database = ctx.link().callback(move |_: InputEvent| {
                let data = get_value_from_ref(&database_ref_2);
                Msg::UpdateDb(data)
            });
            let database = if self.database_directory.is_none() {
                {
                    #[allow(clippy::useless_conversion)]
                    <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                        #[allow(clippy::redundant_clone, unused_braces)]
                        ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                            ::yew::virtual_dom::VTag::__new_input(
                                ::yew::html::IntoPropValue::<
                                    ::std::option::Option<::yew::virtual_dom::AttrValue>,
                                >::into_prop_value(DEFAULT_DB),
                                false,
                                ::yew::html::IntoPropValue::<::yew::html::NodeRef>::into_prop_value(
                                    self.database_ref.clone(),
                                ),
                                ::std::option::Option::None,
                                ::yew::virtual_dom::Attributes::Static(&[["name", "database"]]),
                                ::yew::virtual_dom::listeners::Listeners::Pending(
                                    ::std::boxed::Box::new([
                                        ::yew::html::oninput::Wrapper::__macro_new(input_database),
                                    ]),
                                ),
                            ),
                        ),
                    )
                }
            } else {
                {
                    #[allow(clippy::useless_conversion)]
                    <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                        #[allow(clippy::redundant_clone, unused_braces)]
                        ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                            ::yew::virtual_dom::VTag::__new_input(
                                ::std::option::Option::None,
                                false,
                                ::yew::html::IntoPropValue::<::yew::html::NodeRef>::into_prop_value(
                                    self.database_ref.clone(),
                                ),
                                ::std::option::Option::None,
                                ::yew::virtual_dom::Attributes::Static(&[["name", "database"]]),
                                ::yew::virtual_dom::listeners::Listeners::Pending(
                                    ::std::boxed::Box::new([
                                        ::yew::html::oninput::Wrapper::__macro_new(input_database),
                                    ]),
                                ),
                            ),
                        ),
                    )
                }
            };
            {
                #[allow(clippy::useless_conversion)]
                <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                    #[allow(clippy::redundant_clone, unused_braces)]
                    ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                        ::yew::virtual_dom::VTag::__new_other(
                            ::std::borrow::Cow::<'static, ::std::primitive::str>::Borrowed("form"),
                            ::std::default::Default::default(),
                            ::std::option::Option::None,
                            ::yew::virtual_dom::Attributes::Static(&[["class", "file_info"]]),
                            ::yew::virtual_dom::listeners::Listeners::None,
                            ::yew::virtual_dom::VList::with_children(
                                {
                                    let mut __yew_v = ::std::vec::Vec::new();
                                    __yew_v . push (:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("label") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& [["for" , "problems_directory"]]) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (:: yew :: virtual_dom :: VText :: new (:: yew :: virtual_dom :: AttrValue :: Static ("Carpeta con los problemas")))])) , :: std :: option :: Option :: None))))) ;
                                    ::std::iter::Extend::extend(
                                        &mut __yew_v,
                                        ::std::convert::Into::<::yew::utils::NodeSeq<_, _>>::into(
                                            problem,
                                        ),
                                    );
                                    __yew_v . push (:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("label") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& [["for" , "database"]]) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (:: yew :: virtual_dom :: VText :: new (:: yew :: virtual_dom :: AttrValue :: Static ("Base de datos")))])) , :: std :: option :: Option :: None))))) ;
                                    ::std::iter::Extend::extend(
                                        &mut __yew_v,
                                        ::std::convert::Into::<::yew::utils::NodeSeq<_, _>>::into(
                                            database,
                                        ),
                                    );
                                    __yew_v
                                },
                                ::std::option::Option::None,
                            ),
                        ),
                    ),
                )
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
}
mod home_button {
    use std::marker::PhantomData;
    use yew::{prelude::*, virtual_dom::VChild};
    pub struct With<T> {
        _marker: PhantomData<T>,
    }
    pub enum UpdateMsg {}
    pub struct Props<TProps: PartialEq> {
        pub return_cb: Callback<()>,
        pub props: TProps,
    }
    struct PropsWrapper<TProps: PartialEq> {
        props_wrapper: ::std::option::Option<TProps>,
        return_cb_wrapper: ::std::option::Option<Callback<()>>,
    }
    impl<TProps: PartialEq> ::std::default::Default for PropsWrapper<TProps> {
        fn default() -> Self {
            PropsWrapper::<TProps> {
                props_wrapper: ::std::option::Option::None,
                return_cb_wrapper: ::std::option::Option::None,
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropsBuilderStep_missing_required_prop_props;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropsBuilderStep_missing_required_prop_return_cb;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropsBuilderStepPropsBuilder;
    #[doc(hidden)]
    pub trait PropsBuilderStep {}
    impl PropsBuilderStep for PropsBuilderStep_missing_required_prop_props {}
    impl PropsBuilderStep for PropsBuilderStep_missing_required_prop_return_cb {}
    impl PropsBuilderStep for PropsBuilderStepPropsBuilder {}
    #[doc(hidden)]
    pub struct PropsBuilder<TProps: PartialEq, YEW_PROPS_BUILDER_STEP: PropsBuilderStep> {
        wrapped: ::std::boxed::Box<PropsWrapper<TProps>>,
        _marker: ::std::marker::PhantomData<YEW_PROPS_BUILDER_STEP>,
    }
    impl<TProps: PartialEq> PropsBuilder<TProps, PropsBuilderStep_missing_required_prop_props> {
        #[doc(hidden)]
        pub fn props(
            mut self,
            props: impl ::yew::html::IntoPropValue<TProps>,
        ) -> PropsBuilder<TProps, PropsBuilderStep_missing_required_prop_return_cb> {
            self.wrapped.props_wrapper = ::std::option::Option::Some(props.into_prop_value());
            PropsBuilder {
                wrapped: self.wrapped,
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    impl<TProps: PartialEq> PropsBuilder<TProps, PropsBuilderStep_missing_required_prop_return_cb> {
        #[doc(hidden)]
        pub fn return_cb(
            mut self,
            return_cb: impl ::yew::html::IntoPropValue<Callback<()>>,
        ) -> PropsBuilder<TProps, PropsBuilderStepPropsBuilder> {
            self.wrapped.return_cb_wrapper =
                ::std::option::Option::Some(return_cb.into_prop_value());
            PropsBuilder {
                wrapped: self.wrapped,
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    impl<TProps: PartialEq> PropsBuilder<TProps, PropsBuilderStepPropsBuilder> {
        #[doc(hidden)]
        pub fn build(self) -> Props<TProps> {
            Props::<TProps> {
                props: ::std::option::Option::unwrap(self.wrapped.props_wrapper),
                return_cb: ::std::option::Option::unwrap(self.wrapped.return_cb_wrapper),
            }
        }
    }
    impl<TProps: PartialEq> ::yew::html::Properties for Props<TProps> {
        type Builder = PropsBuilder<TProps, PropsBuilderStep_missing_required_prop_props>;
        fn builder() -> Self::Builder {
            PropsBuilder {
                wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    #[automatically_derived]
    impl<TProps: PartialEq> ::core::marker::StructuralPartialEq for Props<TProps> {}
    #[automatically_derived]
    impl<TProps: ::core::cmp::PartialEq + PartialEq> ::core::cmp::PartialEq for Props<TProps> {
        #[inline]
        fn eq(&self, other: &Props<TProps>) -> bool {
            self.return_cb == other.return_cb && self.props == other.props
        }
    }
    #[automatically_derived]
    impl<TProps: ::core::clone::Clone + PartialEq> ::core::clone::Clone for Props<TProps> {
        #[inline]
        fn clone(&self) -> Props<TProps> {
            Props {
                return_cb: ::core::clone::Clone::clone(&self.return_cb),
                props: ::core::clone::Clone::clone(&self.props),
            }
        }
    }
    impl<T: Component> Component for With<T>
    where
        T::Properties: Clone,
    {
        type Message = UpdateMsg;
        type Properties = Props<T::Properties>;
        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _marker: PhantomData::<T>,
            }
        }
        fn view(&self, ctx: &Context<Self>) -> Html {
            let return_cb = ctx.props().return_cb.clone();
            let return_button = ctx.link().batch_callback(move |_: MouseEvent| {
                return_cb.emit(());
                None
            });
            let t: VChild<T> = VChild::new(ctx.props().props.clone(), NodeRef::default(), None);
            {
                #[allow(clippy::useless_conversion)]
                <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                    ::yew::virtual_dom::VNode::VList(::yew::virtual_dom::VList::with_children(
                        {
                            let mut __yew_v = ::std::vec::Vec::new();
                            __yew_v . push (:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("button") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: Pending (:: std :: boxed :: Box :: new ([:: yew :: html :: onclick :: Wrapper :: __macro_new (return_button)])) , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (:: yew :: virtual_dom :: VText :: new (:: yew :: virtual_dom :: AttrValue :: Static ("Volver al inicio")))])) , :: std :: option :: Option :: None))))) ;
                            ::std::iter::Extend::extend(
                                &mut __yew_v,
                                ::std::convert::Into::<::yew::utils::NodeSeq<_, _>>::into(t),
                            );
                            __yew_v
                        },
                        ::std::option::Option::None,
                    )),
                )
            }
        }
    }
}
mod main_menu {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::rc::Rc;
    use crate::app::invoke;
    use crate::files_info::{Comp as FilesInfo, Paths, DEFAULT_DB};
    use crate::update_db::UpdateDb as Update;
    use crate::view_db::ViewDb as View;
    use crate::{home_button, DB};
    use parse_lib::data::Data;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::to_value;
    use yew::prelude::*;
    use AppType::Start;
    pub struct MainMenu {
        main_app: AppType,
        paths: Paths,
        db: Option<Rc<HashMap<usize, Data>>>,
        error: String,
    }
    pub enum AppType {
        Start,
        Update,
        View,
    }
    pub enum Msg {
        ChangeApps(AppType),
        UpdatePaths(Paths),
        UpdateDb(HashMap<usize, Data>),
        UpdateErr(String),
    }
    pub struct GetJsonArgs {
        #[serde(rename = "jsonPath")]
        json_path: PathBuf,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for GetJsonArgs {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GetJsonArgs",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "jsonPath",
                    &self.json_path,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for GetJsonArgs {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "jsonPath" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"jsonPath" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GetJsonArgs>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GetJsonArgs;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct GetJsonArgs")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<PathBuf>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GetJsonArgs with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(GetJsonArgs {
                            json_path: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<PathBuf> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "jsonPath",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<PathBuf>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("jsonPath") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GetJsonArgs {
                            json_path: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["jsonPath"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GetJsonArgs",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GetJsonArgs>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Component for MainMenu {
        type Message = Msg;
        type Properties = ();
        fn create(ctx: &Context<Self>) -> Self {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Create something"], &[]),
                        lvl,
                        &(
                            "pim_aplicacion_ui::main_menu",
                            "pim_aplicacion_ui::main_menu",
                            "pim-aplicacion/src/main_menu.rs",
                            46u32,
                        ),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            ctx.link().send_future(async move {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(&["Trying to invoke"], &[]),
                            lvl,
                            &(
                                "pim_aplicacion_ui::main_menu",
                                "pim_aplicacion_ui::main_menu",
                                "pim-aplicacion/src/main_menu.rs",
                                48u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                let db = invoke(
                    "get_db_from_json",
                    to_value(&GetJsonArgs {
                        json_path: PathBuf::from(DEFAULT_DB),
                    })
                    .unwrap(),
                )
                .await;
                let db: Result<Result<DB, String>, _> = serde_wasm_bindgen::from_value(db);
                match db {
                    Ok(Ok(db)) => Msg::UpdateDb(db),
                    Ok(Err(err)) => Msg::UpdateErr(err),
                    Err(parse_err) => Msg::UpdateErr({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error parsing response: "],
                            &[::core::fmt::ArgumentV1::new_display(&parse_err)],
                        ));
                        res
                    }),
                }
            });
            Self {
                main_app: AppType::Start,
                paths: Paths::default(),
                db: None,
                error: String::new(),
            }
        }
        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::ChangeApps(app) => {
                    self.main_app = app;
                    true
                }
                Msg::UpdatePaths(paths) => {
                    self.paths = paths;
                    true
                }
                Msg::UpdateDb(db) => {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_debug(&db)],
                                ),
                                lvl,
                                &(
                                    "pim_aplicacion_ui::main_menu",
                                    "pim_aplicacion_ui::main_menu",
                                    "pim-aplicacion/src/main_menu.rs",
                                    83u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.db = Some(Rc::new(db));
                    false
                }
                Msg::UpdateErr(err) => {
                    self.error = err;
                    true
                }
            }
        }
        fn view(&self, ctx: &Context<Self>) -> Html {
            let paths = self.paths.clone();
            let update_cb = ctx.link().callback(Msg::UpdatePaths);
            let main_app = match self.main_app {
                AppType::Start => Self::view_start(ctx),
                AppType::Update => Self::view_update(ctx),
                AppType::View => Self::view_db(ctx),
            };
            {
                #[allow(clippy::useless_conversion)]
                <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                    #[allow(clippy::redundant_clone, unused_braces)]
                    ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                        ::yew::virtual_dom::VTag::__new_other(
                            ::std::borrow::Cow::<'static, ::std::primitive::str>::Borrowed("div"),
                            ::std::default::Default::default(),
                            ::std::option::Option::None,
                            ::yew::virtual_dom::Attributes::Static(&[["id", "container"]]),
                            ::yew::virtual_dom::listeners::Listeners::None,
                            ::yew::virtual_dom::VList::with_children(
                                {
                                    let mut __yew_v = ::std::vec::Vec::new();
                                    __yew_v.push(
                                        ::std::convert::Into::into(
                                            #[allow(clippy::redundant_clone, unused_braces)]
                                            ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                                                ::yew::virtual_dom::VTag::__new_other(
                                                    ::std::borrow::Cow::<
                                                        'static,
                                                        ::std::primitive::str,
                                                    >::Borrowed(
                                                        "p"
                                                    ),
                                                    ::std::default::Default::default(),
                                                    ::std::option::Option::None,
                                                    ::yew::virtual_dom::Attributes::Static(&[]),
                                                    ::yew::virtual_dom::listeners::Listeners::None,
                                                    ::yew::virtual_dom::VList::with_children(
                                                        {
                                                            let mut __yew_v =
                                                                ::std::vec::Vec::new();
                                                            ::std::iter::Extend::extend(
                                                                &mut __yew_v,
                                                                ::std::convert::Into::<
                                                                    ::yew::utils::NodeSeq<_, _>,
                                                                >::into(
                                                                    &self.error
                                                                ),
                                                            );
                                                            __yew_v
                                                        },
                                                        ::std::option::Option::None,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    );
                                    __yew_v . push (:: std :: convert :: Into :: into ({ let __yew_props = { # [allow (clippy :: no_effect)] if false { let _ = | __yew_props : < FilesInfo as :: yew :: html :: Component > :: Properties | { __yew_props . paths ; __yew_props . update_cb ; } ; } < < FilesInfo as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder () . paths (paths) . update_cb (update_cb) . build () } ; :: yew :: virtual_dom :: VChild :: < FilesInfo > :: new (__yew_props , < :: yew :: html :: NodeRef as :: std :: default :: Default > :: default () , :: std :: option :: Option :: None) })) ;
                                    ::std::iter::Extend::extend(
                                        &mut __yew_v,
                                        ::std::convert::Into::<::yew::utils::NodeSeq<_, _>>::into(
                                            main_app,
                                        ),
                                    );
                                    __yew_v
                                },
                                ::std::option::Option::None,
                            ),
                        ),
                    ),
                )
            }
        }
    }
    impl MainMenu {
        fn view_start(ctx: &Context<Self>) -> Html {
            let update_db = ctx
                .link()
                .callback(|_: MouseEvent| Msg::ChangeApps(AppType::Update));
            let view_db = ctx
                .link()
                .callback(|_: MouseEvent| Msg::ChangeApps(AppType::View));
            {
                # [allow (clippy :: useless_conversion)] < :: yew :: virtual_dom :: VNode as :: std :: convert :: From < _ > > :: from (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("div") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& [["id" , "container"]]) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("p") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (:: yew :: virtual_dom :: VText :: new (:: yew :: virtual_dom :: AttrValue :: Static ("¿Qué quieres hacer?")))])) , :: std :: option :: Option :: None)))) , :: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("ul") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("li") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("button") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: Pending (:: std :: boxed :: Box :: new ([:: yew :: html :: onclick :: Wrapper :: __macro_new (update_db)])) , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (:: yew :: virtual_dom :: VText :: new (:: yew :: virtual_dom :: AttrValue :: Static ("Actualizar la base de datos")))])) , :: std :: option :: Option :: None))))])) , :: std :: option :: Option :: None)))) , :: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("li") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("button") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: Pending (:: std :: boxed :: Box :: new ([:: yew :: html :: onclick :: Wrapper :: __macro_new (view_db)])) , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (:: yew :: virtual_dom :: VText :: new (:: yew :: virtual_dom :: AttrValue :: Static ("Ver la base de datos")))])) , :: std :: option :: Option :: None))))])) , :: std :: option :: Option :: None))))])) , :: std :: option :: Option :: None))))])) , :: std :: option :: Option :: None))))
            }
        }
        fn view_update(ctx: &Context<Self>) -> Html {
            let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
            {
                #[allow(clippy::useless_conversion)]
                <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                    ::yew::virtual_dom::VNode::VList(::yew::virtual_dom::VList::with_children(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([::std::convert::Into::into({
                                let __yew_props = {
                                    #[allow(clippy::no_effect)]
                                    if false {
                                        let _ = | __yew_props : < home_button :: With < Update > as :: yew :: html :: Component > :: Properties | { __yew_props . props ; __yew_props . return_cb ; } ;
                                    }
                                    < < home_button :: With < Update > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder () . props (()) . return_cb (return_cb) . build ()
                                };
                                ::yew::virtual_dom::VChild::<home_button::With<Update>>::new(
                                    __yew_props,
                                    <::yew::html::NodeRef as ::std::default::Default>::default(),
                                    ::std::option::Option::None,
                                )
                            })]),
                        ),
                        ::std::option::Option::None,
                    )),
                )
            }
        }
        fn view_db(ctx: &Context<Self>) -> Html {
            let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
            {
                #[allow(clippy::useless_conversion)]
                <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                    ::yew::virtual_dom::VNode::VList(::yew::virtual_dom::VList::with_children(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([::std::convert::Into::into({
                                let __yew_props = {
                                    #[allow(clippy::no_effect)]
                                    if false {
                                        let _ = | __yew_props : < home_button :: With < View > as :: yew :: html :: Component > :: Properties | { __yew_props . props ; __yew_props . return_cb ; } ;
                                    }
                                    < < home_button :: With < View > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder () . props (()) . return_cb (return_cb) . build ()
                                };
                                ::yew::virtual_dom::VChild::<home_button::With<View>>::new(
                                    __yew_props,
                                    <::yew::html::NodeRef as ::std::default::Default>::default(),
                                    ::std::option::Option::None,
                                )
                            })]),
                        ),
                        ::std::option::Option::None,
                    )),
                )
            }
        }
    }
}
mod update_db {
    use yew::prelude::*;
    use crate::files_info::Paths;
    pub struct UpdateDb;
    pub enum Msg {
        ParseFiles,
    }
    pub struct Properties {
        paths: Paths,
    }
    struct PropertiesWrapper {
        paths_wrapper: ::std::option::Option<Paths>,
    }
    impl ::std::default::Default for PropertiesWrapper {
        fn default() -> Self {
            PropertiesWrapper {
                paths_wrapper: ::std::option::Option::None,
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropertiesBuilderStep_missing_required_prop_paths;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct PropertiesBuilderStepPropsBuilder;
    #[doc(hidden)]
    pub trait PropertiesBuilderStep {}
    impl PropertiesBuilderStep for PropertiesBuilderStep_missing_required_prop_paths {}
    impl PropertiesBuilderStep for PropertiesBuilderStepPropsBuilder {}
    #[doc(hidden)]
    pub struct PropertiesBuilder<YEW_PROPS_BUILDER_STEP: PropertiesBuilderStep> {
        wrapped: ::std::boxed::Box<PropertiesWrapper>,
        _marker: ::std::marker::PhantomData<YEW_PROPS_BUILDER_STEP>,
    }
    impl PropertiesBuilder<PropertiesBuilderStep_missing_required_prop_paths> {
        #[doc(hidden)]
        pub fn paths(
            mut self,
            paths: impl ::yew::html::IntoPropValue<Paths>,
        ) -> PropertiesBuilder<PropertiesBuilderStepPropsBuilder> {
            self.wrapped.paths_wrapper = ::std::option::Option::Some(paths.into_prop_value());
            PropertiesBuilder {
                wrapped: self.wrapped,
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    impl PropertiesBuilder<PropertiesBuilderStepPropsBuilder> {
        #[doc(hidden)]
        pub fn build(self) -> Properties {
            Properties {
                paths: ::std::option::Option::unwrap(self.wrapped.paths_wrapper),
            }
        }
    }
    impl ::yew::html::Properties for Properties {
        type Builder = PropertiesBuilder<PropertiesBuilderStep_missing_required_prop_paths>;
        fn builder() -> Self::Builder {
            PropertiesBuilder {
                wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
                _marker: ::std::marker::PhantomData,
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Properties {
        #[inline]
        fn clone(&self) -> Properties {
            Properties {
                paths: ::core::clone::Clone::clone(&self.paths),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Properties {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Properties {
        #[inline]
        fn eq(&self, other: &Properties) -> bool {
            self.paths == other.paths
        }
    }
    impl Component for UpdateDb {
        type Message = Msg;
        type Properties = ();
        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }
        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::ParseFiles => {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(&["Unimplemented"], &[]),
                                lvl,
                                &(
                                    "pim_aplicacion_ui::update_db",
                                    "pim_aplicacion_ui::update_db",
                                    "pim-aplicacion/src/update_db.rs",
                                    27u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    false
                }
            }
        }
        fn view(&self, ctx: &Context<Self>) -> Html {
            let onclick = ctx.link().callback(|_: MouseEvent| Msg::ParseFiles);
            {
                # [allow (clippy :: useless_conversion)] < :: yew :: virtual_dom :: VNode as :: std :: convert :: From < _ > > :: from (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("div") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: None , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (# [allow (clippy :: redundant_clone , unused_braces)] :: std :: convert :: Into :: < :: yew :: virtual_dom :: VNode > :: into (:: yew :: virtual_dom :: VTag :: __new_other (:: std :: borrow :: Cow :: < 'static , :: std :: primitive :: str > :: Borrowed ("button") , :: std :: default :: Default :: default () , :: std :: option :: Option :: None , :: yew :: virtual_dom :: Attributes :: Static (& []) , :: yew :: virtual_dom :: listeners :: Listeners :: Pending (:: std :: boxed :: Box :: new ([:: yew :: html :: onclick :: Wrapper :: __macro_new (onclick)])) , :: yew :: virtual_dom :: VList :: with_children (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: std :: convert :: Into :: into (:: yew :: virtual_dom :: VText :: new (:: yew :: virtual_dom :: AttrValue :: Static ("Actualizar")))])) , :: std :: option :: Option :: None))))])) , :: std :: option :: Option :: None))))
            }
        }
    }
}
mod view_db {
    use yew::prelude::*;
    pub struct ViewDb;
    pub enum Msg {}
    impl Component for ViewDb {
        type Message = Msg;
        type Properties = ();
        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }
        fn view(&self, _ctx: &Context<Self>) -> Html {
            {
                #[allow(clippy::useless_conversion)]
                <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
                    ::yew::virtual_dom::VNode::VList(::yew::virtual_dom::VList::new()),
                )
            }
        }
    }
}
use std::collections::HashMap;
use app::App;
use parse_lib::data::Data;
pub type DB = HashMap<usize, Data>;
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<App>();
}
