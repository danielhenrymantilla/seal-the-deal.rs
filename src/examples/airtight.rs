/// `#[sealed(airtight(doc(disguised = false)))]` case.
///
/// Example of the rendered docs of a
/// <code>[#\[with_seals\]][`seal_the_deal::with_seals`]</code>-annotated `trait` definition.
///
/// The source code for the trait was:
///
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", file!()))]
/// ```
#[seal_the_deal::with_seals]
pub trait AirtightDocDisguisedFalse {
    /// This shall always return `42`.
    #[sealed(airtight, doc(disguised = false))]
    fn method(&self) -> i32 {
        42
    }

    /// This shall always call its arg and then return `42`.
    #[sealed(airtight, doc(disguised = false))]
    fn generic_method<F: FnOnce()>(&self, f: F) -> i32 {
        f();
        42
    }
}
