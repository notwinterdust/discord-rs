use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, FnArg, ItemFn, Lit, Meta, Token, parse_macro_input};

struct CommandArgs {
    name: Option<String>,
    description: Option<String>,
    aliases: Vec<String>,
}

impl Parse for CommandArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;
        let mut aliases = Vec::new();

        while !input.is_empty() {
            let meta: Meta = input.parse()?;
            match meta {
                Meta::NameValue(nv) => {
                    if nv.path.is_ident("name") {
                        if let Expr::Lit(expr_lit) = &nv.value {
                            if let Lit::Str(s) = &expr_lit.lit {
                                name = Some(s.value());
                            }
                        }
                    } else if nv.path.is_ident("description") {
                        if let Expr::Lit(expr_lit) = &nv.value {
                            if let Lit::Str(s) = &expr_lit.lit {
                                description = Some(s.value());
                            }
                        }
                    } else if nv.path.is_ident("aliases") {
                        if let Expr::Array(arr) = &nv.value {
                            for elem in &arr.elems {
                                if let Expr::Lit(expr_lit) = elem {
                                    if let Lit::Str(s) = &expr_lit.lit {
                                        aliases.push(s.value());
                                    }
                                }
                            }
                        }
                    }
                }
                Meta::Path(path) => {
                    if let Some(ident) = path.get_ident() {
                        name = Some(ident.to_string());
                    }
                }
                _ => {}
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(CommandArgs {
            name,
            description,
            aliases,
        })
    }
}

/// Marks a function as a bot command and generates a definition constructor.
///
/// # Example
///
/// ```ignore
/// #[command(description = "Adds two numbers", aliases = ["sum", "plus"])]
/// fn add(ctx: &Context, a: i32, b: i32) {
///     // ...
/// }
///
/// registry.register(add_command()).unwrap();
/// ```
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as CommandArgs);
    let item_fn = parse_macro_input!(item as ItemFn);

    let fn_ident = &item_fn.sig.ident;
    let command_fn_ident = format_ident!("{}_command", fn_ident);

    let cmd_name = args.name.unwrap_or_else(|| fn_ident.to_string());
    let cmd_aliases = args.aliases;
    let description_expr = match args.description {
        Some(d) => quote! { Some(#d.to_string()) },
        None => quote! { None },
    };

    let mut takes_ctx = false;
    let mut arg_extractors = Vec::new();
    let mut call_args = Vec::new();
    let mut param_index = 0usize;

    for (i, input) in item_fn.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(pat_type) = input {
            let ty = &pat_type.ty;
            if i == 0 {
                let is_ctx = match &**ty {
                    syn::Type::Reference(r) => match &*r.elem {
                        syn::Type::Path(p) => {
                            p.path.segments.last().is_some_and(|s| s.ident == "Context")
                        }
                        _ => false,
                    },
                    _ => false,
                };
                if is_ctx {
                    takes_ctx = true;
                    call_args.push(quote! { ctx });
                    continue;
                }
            }

            let var_name = format_ident!("arg_{}", param_index);
            arg_extractors.push(quote! {
                let #var_name = match ctx.arg::<#ty>(#param_index) {
                    Ok(val) => val,
                    Err(_) => return,
                };
            });
            call_args.push(quote! { #var_name });
            param_index += 1;
        }
    }

    let call_expr = if takes_ctx || !call_args.is_empty() {
        quote! { #fn_ident(#(#call_args),*); }
    } else {
        quote! { #fn_ident(); }
    };

    let expanded = quote! {
        #item_fn

        #[must_use]
        pub fn #command_fn_ident() -> commands::CommandDefinition {
            commands::CommandDefinition {
                name: #cmd_name.to_string(),
                aliases: vec![#(#cmd_aliases.to_string()),*],
                description: #description_expr,
                handler: Box::new(|ctx: &commands::Context| {
                    #(#arg_extractors)*
                    #call_expr
                }),
            }
        }
    };

    expanded.into()
}
