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

mod args;

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
    let mut trait_: ItemTrait = parse2(input)?;
    let trait_name = &trait_.ident;
    let mut ret = quote!();
    let mut extra_methods = Vec::<TraitItem>::new();
    let storage = (::core::cell::OnceCell::new(), ::core::cell::OnceCell::new());
    let helper_module_name = |ret: &mut TokenStream2| -> &Ident {
        storage.0.get_or_init(|| {
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
    let helper_module_name_airtight = |ret: &mut TokenStream2| -> &Ident {
        storage.1.get_or_init(|| {
            let module_name = format_ident!("__{trait_name}ඞseal_the_deal_airtight");
            ret.extend(quote!(
                #[allow(warnings, clippy::all)]
                mod #module_name {
                    pub trait Sealed<Seal> {
                        type ඞRet<T>;
                        fn ඞret<T>(_: T) -> Self::ඞRet<T>;
                    }
                    impl<X : ?Sized> Sealed<()> for X {
                        type ඞRet<T> = T;
                        #[inline]
                        fn ඞret<T>(it: T) -> Self::ඞRet<T> {
                            it
                        }
                    }
                }
            ));
            module_name
        })
    };
    for assoc_item in &mut trait_.items {
        // Only handle associated functions (_strictu sensu_, them being methods is not required.)
        let TraitItem::Fn(assoc_func) = assoc_item
        else {
            continue;
        };
        // Only handle functions with a `#[sealed]` annotation on them:
        let sealed_attr_span;
        let sealed_attr_args: args::SealedArgs;
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
                    sealed_attr_args = match &the_attr.meta {
                        | Meta::Path(_) => parse2(quote!())?,
                        | _ => the_attr.parse_args()?,
                    };
                    sealed_attr_span = the_attr.path().span(); // .resolved_at(Span::mixed_site());
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
        assoc_func.attrs.push(parse_quote!(
            #[doc = "\n\
                \n\
                ---\n\
                \n\
                ## Sealed method\n\
                \n\
                This function is deliberately \"sealed\": it shall be impossible to override the \
                default implementation in any kind of `impl` block whatsoever.\
            "]
        ));
        let requested_airtight = sealed_attr_args.requested_airtight().unwrap_or(false);
        if sealed_attr_args.requested_doc_disguised().unwrap_or(requested_airtight) {
            let cfg_doc = quote!(
                all(doc, not(doctest))
            );
            {
                let mut assoc_func = assoc_func.clone();
                assoc_func.attrs.push(parse_quote!(
                    #[cfg(#cfg_doc)]
                ));
                extra_methods.push(TraitItem::Fn(assoc_func));
            }
            assoc_func.attrs.push(parse_quote!(
                #[cfg(not(#cfg_doc))]
            ));
        }
        let generics = &mut assoc_func.sig.generics;
        if requested_airtight.not() {
            /* Strategy:
            mod TraitName_seal_the_deal {
                pub trait Seal<'__> {}
                impl Seal<'_> for () {}
            }
            // and then use the following generics and clauses:
            //           👇
            fn method<'sealed>(…) -> …
            where
                …,
                () : …::Seal<'sealed>, // 👈
            */
            let extra_lifetime = {
                let mut storage = None;
                let lifetime_name: &str = {
                    let mut lifetime_name = "sealed";
                    for i in 0_u64.. {
                        if Iterator::chain(
                                generics.lifetimes(),
                                trait_.generics.lifetimes(),
                            )
                            .any(|lt| lt.lifetime.ident == lifetime_name)
                        {
                            lifetime_name = storage.insert(format!("ඞseal_the_deal__sealed{i}"));
                        } else {
                            break;
                        }
                    }
                    lifetime_name
                };
                Lifetime::new(&format!("'{lifetime_name}"), sealed_attr_span)
            };
            let helper_module_name = helper_module_name(&mut ret);
            generics.params.insert(
                generics.lifetimes().count(),
                parse_quote!( #extra_lifetime ),
            );
            generics.make_where_clause().predicates.push(parse_quote_spanned!(sealed_attr_span=>
                () : #helper_module_name::Seal<#extra_lifetime>
            ));
        } else {
            /* Strategy:
            mod TraitName_seal_the_deal {
                pub trait Seal<U> {
                    type ඞRet<T>;
                    fn ඞret<T>(_: T) -> Self::ඞRet<T>;
                }
                impl<X : ?Sized> Seal<()> for X {
                    type ඞRet<T> = T;
                    #[inline]
                    fn ඞret<T>(it: T) -> Self::ඞRet<T> {
                        it
                    }
                }
            }
            */
            let helper_module_name = helper_module_name_airtight(&mut ret);
            let SealTrait @ _ = quote_spanned!(sealed_attr_span=>
                #helper_module_name::Sealed
            );
            let ret_ty = {
                let ret_ty = &mut assoc_func.sig.output;
                if matches!(ret_ty, ReturnType::Default) {
                    *ret_ty = parse_quote_spanned!(sealed_attr_span=>
                        -> ()
                    );
                }
                if let ReturnType::Type(_, it) = ret_ty { it } else { unreachable!() }
            };
            let fn_body = assoc_func.default.as_mut().unwrap();
            let fn_body_catching_returns = if let Some(async_) = assoc_func.sig.asyncness {
                quote_spanned!(async_.span()=>
                    #async_ #fn_body .await
                )
            } else {
                quote_spanned!(sealed_attr_span=>
                    || -> _ #fn_body ()
                )
            };
            // And then use the following generics and clauses:
            if let Some(first_generic) = generics.type_params_mut().next() {
                /* # If one generic type param already present:
                //
                //            vvvvvvvvvvvvvvvvvvv           vvvvvvvv   v
                fn method<T : …::Seal<impl Sized>, …>(…) -> T::ඞRet< … >
                …
                {
                    T::ඞret(|| -> _ { // 👈
                        …
                    }()) // 👈
                }
                */
                first_generic.bounds.push(parse_quote_spanned!(sealed_attr_span=>
                    #SealTrait<impl ::core::marker::Sized>
                ));
                let FirstT @ _ = &first_generic.ident;
                **ret_ty = parse_quote_spanned!(sealed_attr_span=>
                    #FirstT::ඞRet< #ret_ty >
                );
                let new_body = Stmt::Expr(
                    parse_quote_spanned!(sealed_attr_span=>
                        #FirstT::ඞret( #fn_body_catching_returns )
                    ),
                    None,
                );
                fn_body.stmts.splice(.., [
                    new_body
                ]);
            } else {
                /* # If no generic type params already present:
                //
                //          👇          vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv   v
                fn method<Sealed>(…) -> <() as …::Seal<Sealed>>::ඞRet< … >
                where
                    …,
                    () : …::Seal<Sealed>, // 👈
                {
                    <() as …::Seal<Sealed>>::ඞret(|| -> _ { // 👈
                        …
                    }()) // 👈
                }
                */
                let SealArg @ _ = {
                    let mut storage = None;
                    let param_name: &str = {
                        let mut param_name = "Seal";
                        for i in 0_u64.. {
                            if trait_.generics.type_params().any(|param| param.ident == param_name) {
                                param_name = storage.insert(format!("ඞseal_the_deal__Seal{i}"));
                            } else {
                                break;
                            }
                        }
                        param_name
                    };
                    Ident::new(param_name, sealed_attr_span)
                };
                generics.params.push(parse_quote_spanned!(sealed_attr_span=>
                    #SealArg
                ));
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote_spanned!(sealed_attr_span=>
                        () : #SealTrait<#SealArg>
                    ))
                ;
                **ret_ty = parse_quote_spanned!(sealed_attr_span=>
                    <() as #SealTrait<#SealArg>>::ඞRet< #ret_ty >
                );
                let new_body = Stmt::Expr(
                    parse_quote_spanned!(sealed_attr_span=>
                        <() as #SealTrait<#SealArg>>::ඞret( #fn_body_catching_returns )
                    ),
                    None,
                );
                fn_body.stmts.splice(.., [
                    new_body
                ]);
            }
        }
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
    }
    trait_.items.extend(extra_methods);
    trait_.to_tokens(&mut ret);
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
