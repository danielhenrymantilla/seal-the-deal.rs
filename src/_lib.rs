//! [sealed]: `sealed`
//! [with_seals]: `with_seals`
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
/// # Attribute args
///
/// `#[sealed]` currently accepts _two_, optional, attribute args:
///
/// ## `#[sealed(airtight)]`
///
/// Switches the implementation used by `#[sealed]` from involving a generic lifetime parameter
/// to involving more convoluted type-genericity tricks.
///
/// The author is positive this alternative method shall never allow users to override the method,
/// no matter how smart the compiler might get, since doing so would require `fn` signatures to
/// be capable of trait-resolution-fallback type inference.
///
/// The reason this alternative implementation is only opt-in, is because of a couple:
///
/// ### Caveats
///
///   - Since the method now uses a(n extra) generic type parameter, a previously `dyn`-compatible
///     method would cease to be so.
///
///     Add `where Self : Sized` if you do not want the rest of the trait from becoming
///     `dyn`-incompatible
///
///   - Uglier generated code.
///
///     Indeed, the return of the function [ends up wrapped in an ugly trait associated type][1].
///
///     While most code and users ought to remain blissfully oblivious to this fact, IDEs might
///     reveal the ugly truth to users; moreover, the docs would be quite horrendous as well,
///     which is why `#[sealed]` then defaults to hiding its shenanigans from the rendered docs, and
///     [masquerading as non-`#[sealed] fn`][3] (but for the extra docstring appended to the method,
///     of course!).
///
/// ## `#[sealed(doc(disguised = true/false))]`
///
/// Whether the attribute shall be using certain `cfg(doc)` shenanigans to disable itself when
/// Rust documentation is being rendered, so as to hide the actual clutter/noise from said
/// sheanigans.
///
///   - ### Default behavior
///
///       - For `#[sealed]`, it defaults to `false`: said sheanigans are deemed non-noisy enough
///         not to warrant having to lie in docs;
///
///       - But for `#[sealed(airtight)]`, it defaults to `true`, since the return type of the
///         method otherwise [ends up wrapped in an ugly trait associated type][1].
///
/// Note that no matter the choice of `doc(disguised)`, `#[sealed]` shall always unconditionally
/// append a docstring to the method / associated function [telling users not to override it][2].
///
/// [1]: examples::AirtightDocDisguisedFalse::method()
///
/// [2]: examples::Basic::method()
///
/// [3]: examples::Airtight::method()
///
/// ---
///
/// See [the top-level docs][`crate`] for more info and examples.
pub use ::seal_the_deal_proc_macros::sealed;

#[doc = include_str!("compile_fail_tests.md")]
mod _compile_fail_tests {}


#[cfg_attr(feature = "docs-rs",
    doc(cfg(doc_examples))
)]
#[cfg(doc_examples)]
pub mod examples;
