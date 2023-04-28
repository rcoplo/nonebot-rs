use proc_macro::TokenStream;

use crate::bot_command::{parse_bot_args, parse_bot_command, ParamsMather, ParamsMatherTuple};

use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, TokenStreamExt};
use syn::spanned::Spanned;
use syn::{parse_macro_input, FnArg, NestedMeta, Meta};
use syn::Lit::Str;


mod bot_command;
mod event_arg;

use crate::event_arg::{
    parse_args_and_command, args_to_token,
};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn event(args: TokenStream, input: TokenStream) -> TokenStream {
    // 获取#[event]的参数
    let attrs = parse_macro_input!(args as syn::AttributeArgs);
    // 获取方法
    let method = parse_macro_input!(input as syn::ItemFn);
    // 解析参数
    let (all, bot_command) = parse_args_and_command(&method, attrs);
    let command_items = parse_bot_command(&method, bot_command);
    // 判断是否为async方法
    if method.sig.asyncness.is_none() {
        abort!(&method.sig.span(), "必须是async方法");
    }
    // 判断事件
    let sig_params = &method.sig.inputs;
    if sig_params.is_empty() {
        abort!(&sig_params.span(), "需要事件作为参数");
    }
    let params: Vec<_> = sig_params.iter().collect();
    let ((event_param, matcher_param), param_skip) = {
        let first_param = params.first().expect("第一个参数获取失败");
        let matcher_param = params.get(1).expect("第二个参数获取失败(1)");
        if let (FnArg::Receiver(_), FnArg::Receiver(_)) = (first_param, matcher_param) {
            if params.len() == 2 {
                abort!(&first_param.span(), "需要事件,匹配器作为参数");
            }
            ((*params.get(1).expect("第一个参数获取失败"), *params.get(2).expect("第二个参数获取失败(2)")), 3)
        } else {
            ((*first_param, *matcher_param), 2)
        }
    };
    // 对事件进行匹配
    let event_param = match event_param {
        FnArg::Receiver(_) => abort!(&event_param.span(), "不支持self"),
        FnArg::Typed(pt) => pt,
    };
    let matcher_param = match matcher_param {
        FnArg::Receiver(_) => abort!(&matcher_param.span(), "不支持self"),
        FnArg::Typed(pt) => pt,
    };

    let event_param_pat = event_param.pat.as_ref();
    let event_param_ty = event_param.ty.as_ref();
    let matcher_param_pat = matcher_param.pat.as_ref();
    let matcher_param_ty = matcher_param.ty.as_ref();
    let event_param_ty = quote! {#event_param_ty};
    let matcher_param_ty = quote! {#matcher_param_ty};
    let event_tokens = match event_param_ty.to_string().as_str() {
        "Event" => (
            quote! {::nonebot_rs::prelude::Event},
        ),
        "MessageEvent" => (
            quote! {::nonebot_rs::prelude::MessageEvent},
        ),
        "PrivateMessageEvent" => (
            quote! {::nonebot_rs::prelude::PrivateMessageEvent},
        ),
        "GroupMessageEvent" => (
            quote! {::nonebot_rs::prelude::GroupMessageEvent},
        ),
        "NoticeEvent" => (
            quote! {::nonebot_rs::prelude::NoticeEvent},
        ),
        "RequestEvent" => (
            quote! {::nonebot_rs::prelude::RequestEvent},
        )
        ,
        "MetaEvent" => (
            quote! {::nonebot_rs::prelude::MetaEvent},
        ),
        "NbEvent" => (
            quote! {::nonebot_rs::prelude::NbEvent},
        ),
        t => abort!(
            event_param.span(),
            format!("未知的参数类型 {}, 事件必须作为&self下一个参数(或第一个参数), ", t),
        ),
    };

    match matcher_param_ty.to_string().as_str() {
        "Matcher < Event >" => (
            quote! {::nonebot_rs::prelude::Matcher<Event>},
        ),
        "Matcher < MessageEvent >" => (
            quote! {::nonebot_rs::prelude::Matcher<MessageEvent>},
        ),
        "Matcher < PrivateMessageEvent >" => (
            quote! {::nonebot_rs::prelude::Matcher<PrivateMessageEvent>},
        ),
        "Matcher < GroupMessageEvent >" => (
            quote! {::nonebot_rs::prelude::Matcher<GroupMessageEvent>},
        ),
        "Matcher < NoticeEvent >" => (
            quote! {::nonebot_rs::prelude::Matcher<NoticeEvent>},
        ),
        "Matcher < RequestEvent >" => (
            quote! {::nonebot_rs::prelude::Matcher<RequestEvent>},
        )
        ,
        "Matcher < MetaEvent >" => (
            quote! {::nonebot_rs::prelude::Matcher<MetaEvent>},
        ),
        "Matcher < NbEvent >" => (
            quote! {::nonebot_rs::prelude::Matcher<NbEvent>},
        ),
        t => abort!(
            event_param.span(),
            format!("未知的参数类型 {}, 匹配器必须作为&self,事件下一个参数,", t),
        ),
    };
    let pms = parse_bot_args(&method, &params[param_skip..params.len()], command_items);
    let event_trait_name = event_tokens.0;
    // 生成代码
    // gen token stream
    let ident = &method.sig.ident;

    // gen trait
    let block = &method.block;
    let build = if all.is_empty() & pms.is_none() {
        quote! {
            #[allow(non_camel_case_types)]
            pub struct #ident {}
            #[::nonebot_rs::async_trait]
            impl ::nonebot_rs::prelude::Handler<#event_trait_name> for #ident {
                fn match_(&mut self, _: &mut #event_trait_name) -> bool {
                    true
                }
                async fn handle(&self, #event_param_pat: #event_param_ty,#matcher_param_pat: #matcher_param_ty) #block
            }
        }
    } else {
        match event_param_ty.to_string().as_str() {
            "MessageEvent" => (),
            "GroupMessageEvent" => (),
            "PrivateMessageEvent" => (),
            _ => abort!(
                &method.sig.span(),
                "event 的参数只支持消息类型事件 [MessageEvent,PrivateMessageEvent,GroupMessageEvent]"
            ),
        }
        let args_vec = args_to_token(all);

        if pms.is_none() {
            quote! {
                #[allow(non_camel_case_types)]
                pub struct #ident {}
                #[::nonebot_rs::async_trait]
                impl ::nonebot_rs::prelude::Handler<#event_trait_name> for #ident {
                    fn match_(&mut self, event: &mut #event_trait_name) -> bool {
                        if !::nonebot_rs::prelude::match_event_args_all(#args_vec, event.into()){
                            return false;
                        }
                        let mut matcher = ::nonebot_rs::prelude::CommandMatcher::new(event.get_message_chain());
                        if matcher.not_blank(){
                            return false;
                        }
                        true
                    }
                    async fn handle(&self, #event_param_pat: #event_param_ty,#matcher_param_pat: #matcher_param_ty) {
                         self.raw(#event_param_pat,#matcher_param_pat).await;
                    }
                }
                impl #ident {
                    async fn raw(&self, #event_param_pat: #event_param_ty,#matcher_param_pat: #matcher_param_ty) #block
                }
            }
        } else {
            let mut p_pats = quote! {};
            let mut command_params_in_raw = quote! {};
            let mut gets = quote! {};
            for x in pms.expect("匹配出错") {
                match x {
                    ParamsMather::Command(command) => {
                        gets.append_all(quote! {
                            if !matcher.match_command(#command) {
                                return false;
                            }
                        });
                    }
                    ParamsMather::Params(pat, ty) => {
                        p_pats.append_all(quote! {
                           self.#pat.clone(),
                        });
                        command_params_in_raw.append_all(quote! {
                           #pat: #ty,
                        });

                        gets.append_all(quote! {
                            let #pat: #ty = match ::nonebot_rs::prelude::matcher_get::<#ty>(&mut matcher) {
                                Some(value) => value,
                                None => return false,
                            };
                            self.#pat = #pat;
                        });
                    }
                    ParamsMather::Multiple(multiple) => {
                        let mut mme = quote! {};
                        let mut pp = vec![];
                        for x in &multiple {
                            match x {
                                ParamsMatherTuple::Command(name) => {
                                    mme.append_all(quote! {
                                        ::nonebot_rs::prelude::TupleMatcherElement::Command(#name),
                                    });
                                }
                                ParamsMatherTuple::Params(p, t) => {
                                    mme.append_all(quote! {
                                        ::nonebot_rs::prelude::TupleMatcherElement::Param,
                                    });
                                    pp.push((*p, *t));
                                }
                            }
                        }
                        gets.append_all(quote! {
                            let mut ps = if let Some(ps) = matcher.tuple_matcher(vec![#mme]) {
                                ps
                            } else {
                                return false;
                            };
                            ps.reverse();
                        });
                        let len = pp.len();
                        gets.append_all(quote! {
                            if ps.len() != #len {
                                return false;
                            }
                        });
                        for (pat, ty) in pp {
                            p_pats.append_all(quote! {
                              self.#pat.clone(),
                            });
                            command_params_in_raw.append_all(quote! {
                                #pat: #ty,
                            });
                            gets.append_all(quote! {
                                    let #pat: #ty = if let Some(np) = ps.pop() {
                                        let sub_matcher = ::nonebot_rs::prelude::TupleMatcher::new(np);
                                        match ::nonebot_rs::prelude::tuple_matcher_get::<#ty>(sub_matcher) {
                                            Some(value) => value,
                                            None => return false,
                                        }
                                    } else {
                                        return false;
                                    };
                                    self.#pat = #pat;
                            });
                        }
                    }
                }
            }

            quote! {
                #[allow(non_camel_case_types)]
                #[derive(Default)]
                pub struct #ident {
                    #command_params_in_raw
                }

                #[::nonebot_rs::async_trait]
                impl ::nonebot_rs::prelude::Handler<#event_trait_name> for #ident {
                    fn match_(&mut self, event: &mut #event_trait_name) -> bool {
                        if !::nonebot_rs::prelude::match_event_args_all(#args_vec, event.into()){
                            return false;
                        }
                        let mut matcher = ::nonebot_rs::prelude::CommandMatcher::new(event.get_message_chain());
                        #gets
                        if matcher.not_blank(){
                            return false;
                        }
                        true
                    }
                    async fn handle(&self, #event_param_pat: #event_param_ty,#matcher_param_pat: #matcher_param_ty) {
                        self.raw(#event_param_pat,#matcher_param_pat,#p_pats).await;
                    }

                }
                impl #ident {
                    async fn raw(&self, #event_param_pat: #event_param_ty,#matcher_param_pat: #matcher_param_ty, #command_params_in_raw) #block
                }
            }
        }
    };

    build.into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn scheduler(args: TokenStream, input: TokenStream) -> TokenStream {
    // 获取#[event]的参数
    let attrs = parse_macro_input!(args as syn::AttributeArgs);
    // 获取方法
    let method = parse_macro_input!(input as syn::ItemFn);
    // 判断是否为async方法
    if method.sig.asyncness.is_none() {
        abort!(&method.sig.span(), "必须是async方法");
    }

    // 判断事件
    let sig_params = &method.sig.inputs;
    if sig_params.is_empty() {
        abort!(&sig_params.span(), "需要bot作为参数");
    }

    let bot_params = match sig_params.first() {
        None => { abort!(&sig_params.span(), "需要bot作为参数"); }
        Some(bot) => {
            match bot {
                FnArg::Receiver(_) => { abort!(&sig_params.span(), "第一个参数不能是self"); }
                FnArg::Typed(t) => { t }
            }
        }
    };
    let block = &method.block;
    let mut cron = String::new();
    match attrs.first() {
        None => { abort!(&method.span(), "必须要一个参数") }
        Some(nm) => {
            if let NestedMeta::Meta(meta) = nm {
                if let Meta::NameValue(nv) = meta {
                    if nv.path.segments.len() != 1 {
                        abort!(&nv.path.span(), "表达式有且只能有一个片段");
                    }
                    let ident = &nv.path.segments.first().unwrap().ident;
                    let ident_name = nv.path.segments.first().unwrap().ident.to_string();
                    match ident_name.as_str() {
                        "cron" => match &nv.lit {
                            Str(value) => {
                                cron.push_str(&value.value());
                            }
                            _ => abort!(&ident.span(), "cron只支持字符串类型参数值"),
                        },
                        _ => abort!(&ident.span(), "不支持的参数名称"),
                    }
                }
            } else {
                abort!(&nm.span(), "必须要一个参数")
            }
        }
    }

    let ident = method.sig.ident;
    let name = format!("{}", ident);
    let bot_params_pat = bot_params.pat.as_ref();
    let bot_params_ty = bot_params.ty.as_ref();
    let bot_params_ty_ = quote! {#bot_params_ty};
    match bot_params_ty_.to_string().as_str() {
        "Arc < Bot >" => {}
        _ => { abort!(&bot_params.span(), "必须是Arc<Bot>") }
    }

    quote!(
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct #ident;

        impl ::nonebot_rs::prelude::ScheduledJob for #ident {
            fn name(&self) -> String {
                #name.to_string()
            }
            fn cron(&self) -> String {
                #cron.to_string()
            }
            fn call(&self, #bot_params_pat: #bot_params_ty) -> std::pin::Pin<Box<dyn std::future::Future<Output=()> + Send + 'static>> {
                let r = self.clone();
                Box::pin(async move{
                        r.raw(#bot_params_pat).await;
                })
            }
        }
        impl #ident {
            async fn raw(&self,#bot_params_pat: #bot_params_ty) #block
        }
    ).into()
}

