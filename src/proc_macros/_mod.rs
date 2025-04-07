//! Crate not intended for direct use.
//! Use https:://docs.rs/seal-the-deal instead.
// Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template
#![allow(nonstandard_style, unused_imports, unused_braces)]

use ::core::{
    mem,
    ops::Not as _,
};
use ::proc_macro::{
    TokenStream,
};
use ::proc_macro2::{
    Span,
    TokenStream as TokenStream2,
    TokenTree as TT,
};
use ::quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use ::syn::{*,
    parse::{Parse, Parser, ParseStream},
    punctuated::Punctuated,
    Result, // Explicitly shadow it
    spanned::Spanned,
};

///
#[proc_macro_attribute] pub
fn with_seals(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream
{
    with_seals_impl(args.into(), input.into())
     // .map(|ret| { println!("{}", ret); ret })
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`#[seal_the_deal::{{with_seals,sealed}}]`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.to_compile_error()
        })
        .into()
}
#[derive(PartialEq)]
enum Retain { Yes, Not }

fn with_seals_impl(
    args: TokenStream2,
    input: TokenStream2,
) -> Result<TokenStream2>
{
    // By default deny any attribute present.
    let _: parse::Nothing = parse2(args)?;
    let mut input: ItemTrait = parse2(input)?;
    let mut ret = quote!();
    let storage = ::core::cell::OnceCell::new();
    let mut helper_module_name = || -> &Ident {
        storage.get_or_init(|| {
            let trait_name = &input.ident;
            let module_name = format_ident!("__{trait_name}ඞseal_the_deal");
            ret.extend(quote!(
                #[allow(warnings, clippy::all)]
                mod #module_name {
                    pub trait Seal<'__> {}
                    impl Seal<'_> for () {}
                }
            ));
            module_name
        })
    };
    for assoc_item in &mut input.items {
        // Only handle associated functions (_strictu sensu_, them being methods is not required.)
        let TraitItem::Fn(assoc_func) = assoc_item
        else {
            continue;
        };
        let sealed_attr_span;
        // Only handle functions with a `#[sealed]` annotation on them:
        {
            let sealed_attrs = &mut vec![];
            assoc_func.attrs.retain(|attr| Retain::Yes == if attr.path().is_ident("sealed") {
                sealed_attrs.push(attr.clone());
                Retain::Not
            } else {
                Retain::Yes
            });
            match &sealed_attrs[..] {
                // Skip if missing arg,
                | _no_sealed_attr_found @ [] => continue,
                // Reject extraneous `…` in `#[sealed(…)]`
                | [the_attr] => {
                    the_attr.meta.require_path_only()?;
                    sealed_attr_span = the_attr.path().span().resolved_at(Span::mixed_site());
                },
                // Reject duplicate `#[sealed]`s.
                | [_, redundant_attr, ..] => {
                    return Err(Error::new_spanned(redundant_attr, "duplicate attribute"));
                },
            }
            if let Some(semicolon) = assoc_func.semi_token {
                return Err(Error::new_spanned(
                    semicolon,
                    "sealing a method with no default implementation seems… unwise.",
                ));
            }
        }
        let generics = &mut assoc_func.sig.generics;
        let extra_lifetime = {
            let mut storage = None;
            let lifetime_name: &str = {
                let mut lifetime_name = "sealed";
                for i in 0_u64.. {
                    if generics.lifetimes().any(|lt| lt.lifetime.ident == lifetime_name) {
                        lifetime_name = storage.insert(format!("ඞseal_the_deal__sealed{i}"));
                    } else {
                        break;
                    }
                }
                lifetime_name
            };
            Lifetime::new(&format!("'{lifetime_name}"), sealed_attr_span)
        };
        let helper_module_name = helper_module_name();
        generics.params.insert(
            generics.lifetimes().count(),
            parse_quote!( #extra_lifetime ),
        );
        generics.make_where_clause().predicates.push(parse_quote_spanned!(sealed_attr_span=>
            () : #helper_module_name::Seal<#extra_lifetime>
        ));
        // span hacks to improve diagnostics:
        // When "lifetimes in impl do not match this method in trait", the diagnostic want to span
        // over the whole `< ... >` generics of the trait method.
        // That is, they start at `<` and end at `>`.
        //
        // We would like for `#[sealed]` to be "blamed" when this occurs, but since in other
        // scenarios there could be a legitimate issue with pre-existing generics, we would like for
        // the blame to end at `>`, if any was already there.
        //
        // Hence why we unconditionally override the span of `<` to that of our `#[sealed]`, whilst
        // only overriding that of `>` when missing (since otherwise it falls back to
        // `Span::{call,mixed}_site()`, i.e., that of `#[with_seals]`).
        generics.lt_token = Some(Token![<](sealed_attr_span));
        generics.gt_token.get_or_insert_with(|| Token![>](sealed_attr_span));
        assoc_func.attrs.push(parse_quote!(
            #[doc = "\
                \n\
                ---\n\
                \n\
                ## Sealed method\n\
                \n\
                This function is deliberately \"sealed\": it shall be impossible to override the \
                default implementation in any kind of `impl` block whatsoever.\n\
                \n  \
                  - (with the current implementation, an attempt to do so will fail with a \
                    \"lifetimes do not match method in trait\" kind of error.)\
            "]
        ));
    }
    input.to_tokens(&mut ret);
    Ok(ret)
}

#[proc_macro_attribute] pub
fn sealed(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream
{
    sealed_impl(args.into(), input.into())
    //  .map(|ret| { println!("{}", ret); ret })
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`#[seal_the_deal::sealed]`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.to_compile_error()
        })
        .into()
}

fn sealed_impl(
    args: TokenStream2,
    input: TokenStream2,
) -> Result<TokenStream2>
{
    // By default deny any attribute present.
    let _: parse::Nothing = parse2(args)?;
    let _: TraitItemFn = parse2(input)?;
    Err(Error::new(
        Span::mixed_site(),
        "missing `#[with_seals]` annotation on the enscoping `trait`.",
    ))
}
