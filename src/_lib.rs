#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![doc(html_logo_url = "https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21")]
#![cfg_attr(feature = "docs-rs", feature(doc_cfg))]

/// Attribute required on the enscoping `trait` definition to enable usage of the
/// <code>[#\[sealed\]][`sealed`]</code> attribute on its associated functions (with default impls).
///
/// See [the top-level docs][`crate`] for more info and examples.
///
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
/// <img height="150px" src="https://gist.github.com/user-attachments/assets/83e97064-6213-4287-9975-57ed98117a21"/>
///
pub use ::seal_the_deal_proc_macros::with_seals;

/// Attribute to use on the `trait` methods (or associated functions) that you wish to "seal", a.k.a.,
/// render them `final`.
///
/// # Usage
///
/// This is actually a phony attribute: **the <code>[#\[with_seals\]][with_seals]</code> attribute
/// is required on the enscoping `trait`**.
///
/// Indeed, <code>[#\[with_seals\]][`with_seals`]</code> is the actual attribute doing the work:
/// `#[sealed]` are just syntactical annotations / markers for it. Which is why the `#[sealed]`
/// attribute does not need to be imported in practice.
///
/// See [the top-level docs][`crate`] for more info and examples.
pub use ::seal_the_deal_proc_macros::sealed;

#[doc = include_str!("compile_fail_tests.md")]
mod _compile_fail_tests {}

/// An example of the rendered docs of a <code>[#\[with_seals\]][`with_seals`]</code>-`trait`
/// definition.
///
/// The source code for the trait was:
///
/// ```rust
/// /// <these very docs>
/// #[::seal_the_deal::with_seals]
/// pub trait Example {
///     #[sealed]
///     /// This shall always return `42`.
///     fn method(&self) -> i32 {
///         42
///     }
/// }
/// ```
#[doc(cfg(docs))]
#[cfg(feature = "docs-rs")]
#[with_seals]
pub trait Example {
    #[sealed]
    /// This shall always return `42`.
    fn method(&self) -> i32 {
        42
    }
}
