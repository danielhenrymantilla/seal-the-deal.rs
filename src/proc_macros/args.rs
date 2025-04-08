use proc_macro2::extra::DelimSpan;

use super::*;

/// ```rust ,ignore
/// #[sealed(
///     // Optional
///     airtight,
///     // Optional
///     doc(disguised = <true | false>),
/// )]
/// ```
#[derive(Default)]
pub(crate)
struct SealedArgs {
    pub(crate) airtight: Option<kw::airtight>,
    pub(crate) doc_disguised: Option<(bool, kw::doc, token::Paren)>,
    //                                      ^^^^^^^^^^^^^^^^^^^^^
    //                                    used for spanning purposes
}

mod kw {
    ::syn::custom_keyword!(airtight);
    ::syn::custom_keyword!(doc);
    ::syn::custom_keyword!(disguised);
}

impl Parse for SealedArgs {
    fn parse(input: ParseStream<'_>) -> Result<SealedArgs> {
        || -> Result<Self> {
            let mut ret = Self::default();
            while input.is_empty().not() {
                let peeker = input.lookahead1();
                match () {
                    | _case if peeker.peek(kw::airtight) => {
                        if ret.airtight.is_some() {
                            return Err(input.error("duplicate arg"));
                        }
                        ret.airtight = Some(input.parse().unwrap());
                    },
                    | _case if peeker.peek(kw::doc) => {
                        if ret.doc_disguised.is_some() {
                            return Err(input.error("duplicate arg"));
                        }
                        let doc: kw::doc = input.parse().unwrap();
                        {
                            let contents;
                            let parens = parenthesized!(contents in input);
                            let input = contents;
                            let _: kw::disguised = input.parse()?;
                            let _: Token![=] = input.parse()?;
                            let lit_bool: LitBool = input.parse()?;
                            ret.doc_disguised = Some((
                                lit_bool.value,
                                doc,
                                parens,
                            ));
                        }
                    },
                    | _default => return Err(peeker.error()),
                }
                let _: Option<Token![,]> = input.parse()?;
            }
            Ok(ret)
        }().map_err(|mut err| {
            err.combine(Error::new_spanned(
                &err.to_compile_error(),
                "\
                    \n\
                    usage:\n  \
                    #[sealed]\n\
                    or:\n  \
                    #[sealed(\n    \
                        // Optional\n    \
                        airtight,\n    \
                        // Optional\n    \
                        doc(disguised = true/false),\n  \
                    )]\
                ",
            ));
            err
        })
    }
}

impl SealedArgs {
    pub(crate)
    fn requested_airtight(&self) -> Option<bool> {
        self.airtight.as_ref().map(|_| true)
    }

    pub(crate)
    fn requested_doc_disguised(&self) -> Option<bool> {
        self.doc_disguised.as_ref().map(|&(b, ..)| b)
    }
}
