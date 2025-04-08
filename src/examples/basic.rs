/// `#[sealed]` case.
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
pub trait Basic {
    /// This shall always return `42`.
    #[sealed]
    // same as #[sealed(doc(disguised = false))]
    fn method(&self) -> i32 {
        42
    }
}
