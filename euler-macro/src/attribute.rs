use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, ItemFn, LitInt, LitStr, Token};

#[allow(dead_code)]
pub(crate) struct Args {
    number: LitInt,
    input_file: Option<LitStr>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<LitInt, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            number: vars[0].clone(),
            input_file: None,
        })
    }
}

pub(crate) fn problem_inner(args: Args, f: ItemFn) -> TokenStream {
    let mod_name = Ident::new(&format!("__problem_{}", &args.number), Span::call_site());
    let struct_name = Ident::new(&format!("__Problem{}", &args.number), Span::call_site());
    let number = args.number;
    let fn_name = &f.sig.ident;
    quote! {
        pub(crate) mod #mod_name {
            use super::*;

            #[derive(Clone, Copy, Debug)]
            pub(crate) struct #struct_name;

            impl euler_trait::Problem for #struct_name {
                fn solve(&self) -> Box<dyn std::fmt::Display> {
                    Box::new(#fn_name())
                }

                fn problem(&self) -> usize {
                    #number
                }
            }

            euler_trait::inventory::submit!(euler_trait::_Problem(&#struct_name as *const _));

            #f
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn trybuild() {
        let t = trybuild::TestCases::new();
        t.pass("tests/01_easy.rs");
    }
}
