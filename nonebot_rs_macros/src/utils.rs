use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use regex::internal::Input;
use syn::{FnArg, Pat, Type};
use syn::spanned::Spanned;

pub fn event_param_match(event_param: &FnArg) -> Option<(&Pat, &Type)> {
    let event_param = match event_param {
        FnArg::Receiver(_) => abort!(&event_param.span(), "不支持self"),
        FnArg::Typed(pt) => pt,
    };
    let event_param_pat = event_param.pat.as_ref();
    let event_param_ty = event_param.ty.as_ref();
    let event_param_ty_ = quote! {#event_param_ty};
    match event_param_ty_.to_string().as_str() {
        "Event" => {}
        "MessageEvent" => {}
        "PrivateMessageEvent" => {}
        "GroupMessageEvent" => {}
        "NoticeEvent" => {}
        "RequestEvent" => {}
        "MetaEvent" => {}
        "NbEvent" => {}
        _ => return None,
    };
    Some((event_param_pat, event_param_ty))
}

pub fn matcher_param_match(matcher_param: &FnArg) -> Option<(&Pat, &Type)> {
    let matcher_param = match matcher_param {
        FnArg::Receiver(_) => abort!(&matcher_param.span(), "不支持self"),
        FnArg::Typed(pt) => pt,
    };
    
    let matcher_param_pat = matcher_param.pat.as_ref();
    let matcher_param_ty = matcher_param.ty.as_ref();
    
    let matcher_param_ty_ = quote! {#matcher_param_ty};
    match matcher_param_ty_.to_string().as_str() {
        "& mut Matcher < Event >" => {}
        "& mut Matcher < MessageEvent >" => {}
        "& mut Matcher < PrivateMessageEvent >" => {}
        "& mut Matcher < GroupMessageEvent >" => {}
        "& mut Matcher < NoticeEvent >" => {}
        "& mut Matcher < RequestEvent >" => {}
        "& mut Matcher < MetaEvent >" => {}
        "& mut Matcher < NbEvent >" => {}
        _ => return None,
    };
    Some((matcher_param_pat, matcher_param_ty))
}

pub fn state_match(state_param: &FnArg) -> Option<(&Pat, &Type)> {
    let state_param = match state_param {
        FnArg::Receiver(_) => abort!(&state_param.span(), "不支持self"),
        FnArg::Typed(pt) => pt,
    };
    let state_param_pat = state_param.pat.as_ref();
    let state_param_ty = state_param.ty.as_ref();
    
    let state_param_ty_ = quote! {#state_param_ty};
    match state_param_ty_.to_string().as_str() {
        "& mut State" => {
            Some((state_param_pat, state_param_ty))
        }
        _ => {
            return None;
        }
    }
}

pub fn params_command<'a>(params: &'a Vec<&'a FnArg>) -> (Option<(&'a Pat, &'a Type)>, (&'a Pat, &'a Type), (&'a Pat, &'a Type), usize) {
    let first_fn_1 = params.first().expect("参数过少");
    if params.len() < 2 {
        abort!(first_fn_1.span(),
            format!("参数过少,必须有状态(可选),事件,匹配器作为参数"),
        )
    }
    match state_match(first_fn_1) {
        None => {
            match event_param_match(first_fn_1) {
                None => {
                    abort!(first_fn_1.span(),
                        format!("未知的参数类型, 事件必须作为第一个参数, "),
                    );
                }
                Some((event_pat, event_ty)) => {
                    let (matcher_pat, matcher_ty) = match matcher_param_match(params.get(1).expect("第二个参数获取失败(1)")) {
                        None => {
                            abort!(first_fn_1.span(),
                            format!("未知的参数类型, 匹配器必须作为第二个参数, "),
                            );
                        }
                        Some((pat, ty)) => {
                            (pat, ty)
                        }
                    };
                    (None, (event_pat, event_ty), (matcher_pat, matcher_ty), 2)
                }
            }
        }
        Some((state_pat, state_ty)) => {
            match event_param_match(params.get(1).expect("第二个参数获取失败(2)")) {
                None => {
                    abort!(first_fn_1.span(),
                    format!("未知的参数类型,事件必须作为第二个参数, "),
                    );
                }
                Some((event_pat, event_ty)) => {
                    let (matcher_pat, matcher_ty) = match matcher_param_match(params.get(2).expect("第三个参数获取失败(1)")) {
                        None => {
                            abort!(first_fn_1.span(),
                            format!("未知的参数类型, 匹配器必须作为第三个参数, "),
                            );
                        }
                        Some((pat, ty)) => {
                            (pat, ty)
                        }
                    };
                    (Some((state_pat, state_ty)), (event_pat, event_ty), (matcher_pat, matcher_ty), 3)
                }
            }
        }
    }
}