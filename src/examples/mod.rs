extern crate self as seal_the_deal;

include!("airtight.rs");
include!("airtight_disguised.rs");
include!("basic.rs");
include!("basic_disguised.rs");

/// Examples showcasing compiler diagnostics when attempting to override a
/// [`#[sealed]`][crate::sealed] method.
///
/// # [`Basic`]
///
/// ```rust ,compile_fail
/// struct Naive;
///
/// impl ::seal_the_deal::examples::Basic for Naive {
///     fn method(&self) -> i32 { // naïve
///         0
///     }
/// }
/// ```
///
/// They'd then get:
///
/// ```rust ,ignore
/// # /*
/// error[E0195]: lifetime parameters or bounds on method `method` do not match the trait declaration
///  --> src/examples/mod.rs:20:14
///   |
/// 8 |     fn method(&self) -> i32 { // naïve
///   |              ^ lifetimes do not match method in trait
/// # */
/// ```
///
/// # [`Basic`], adversarial
///
/// ```rust ,compile_fail
/// struct Evil;
///
/// impl ::seal_the_deal::examples::Basic for Evil {
///     fn method<'huh>(&self) -> i32 { // evil
///         0
///     }
/// }
/// ```
///
/// They'd then get:
///
/// ```rust ,ignore
/// # /*
/// error[E0195]: lifetime parameters or bounds on method `method` do not match the trait declaration
///  --> src/examples/mod.rs:42:14
///   |
/// 8 |     fn method<'huh>(&self) -> i32 { // evil
///   |              ^^^^^^ lifetimes do not match method in trait
/// # */
/// ```
///
/// # [`Airtight`]
///
/// ```rust ,compile_fail
/// struct Naive;
///
/// impl ::seal_the_deal::examples::Airtight for Naive {
///     fn method(&self) -> i32 { // naïve
///         0
///     }
/// }
/// ```
///
/// They'd then get:
///
/// ```rust ,ignore
/// # /*
/// error[E0049]: method `method` has 0 type parameters but its trait declaration has 1 type parameter
///   --> src/examples/mod.rs:39:14
///    |
/// 8  |       fn method(&self) -> i32 { // naïve
///    |                ^ found 0 type parameters
///    |
///   ::: /Users/dhm/git/dhm/seal-the-deal.rs/src/examples/airtight_disguised.rs:13:7
///    |
/// 13 |       #[sealed(airtight)] // same as #[sealed(airtight, doc(disguised = true))]
///    |  _______-
/// 14 | |     /// This shall always return `42`.
/// 15 | |     fn method(&self) -> i32 {
///    | |______- expected 1 type parameter
/// # */
/// ```
///
/// # [`Airtight`] adversarial
///
/// ```rust ,compile_fail
/// struct Evil;
///
/// impl ::seal_the_deal::examples::Airtight for Evil {
///     fn method<T>(&self) -> i32 { // evil
///         0
///     }
/// }
/// ```
///
/// They'd then get:
///
/// ```rust ,ignore
/// # /*
/// error[E0053]: method `method` has an incompatible type for trait
///  --> src/examples/mod.rs:58:28
///   |
/// 8 |     fn method<T>(&self) -> i32 { // evil
///   |                            ^^^
///   |                            |
///   |                            expected associated type, found `i32`
///   |                            help: change the output type to match the trait: `<() as examples::__Airtightඞseal_the_deal_airtight::Sealed<T>>::ඞRet<i32>`
///   |
///   = note: expected signature `fn(&Evil) -> <() as examples::__Airtightඞseal_the_deal_airtight::Sealed<T>>::ඞRet<i32>`
///              found signature `fn(&Evil) -> i32`
/// # */
/// ```
///
/// # [`Airtight`]'s `.generic_method()`
///
/// ```rust ,compile_fail
/// struct Naive;
///
/// impl ::seal_the_deal::examples::Airtight for Naive {
///     fn generic_method<F: FnOnce()>(&self, f: F) -> i32 { // naïve
///         0
///     }
/// }
/// ```
///
/// They'd then get:
///
/// ```rust ,ignore
/// # /*
///  error[E0049]: method `generic_method` has 1 type parameter but its trait declaration has 2 type parameters
///    --> src/examples/mod.rs:76:23
///     |
///  8  |       fn generic_method<F: FnOnce()>(&self, f: F) -> i32 { // naïve
///     |                         ^ found 1 type parameter
///     |
///    ::: /Users/dhm/git/dhm/seal-the-deal.rs/src/examples/airtight_disguised.rs:19:7
///     |
///  19 |       #[sealed(airtight)]
///     |  _______-
///  20 | |     /// This shall always call its arg and then return `42`.
///  21 | |     fn generic_method<F: FnOnce()>(&self, f: F) -> i32 {
///     | |______- expected 2 type parameters
/// # */
/// ```
///
/// ## Stubborn
///
/// They might be tempted to keep trying:
///
/// ```rust ,compile_fail
/// struct Stubborn;
///
/// impl ::seal_the_deal::examples::Airtight for Stubborn {
///     fn generic_method<F: FnOnce(), Huh>(&self, f: F) -> i32 { // stubborn
///         0
///     }
/// }
/// ```
///
/// They'd then get:
///
/// ```rust ,ignore
/// # /*
/// error[E0643]: method `generic_method` has incompatible signature for trait
///   --> src/examples/mod.rs:108:36
///    |
/// 8  |     fn generic_method<F: FnOnce(), Huh>(&self, f: F) -> i32 { // naïve
///    |                                    ^^^ expected `impl Trait`, found generic parameter
///    |
///   ::: /Users/dhm/git/dhm/seal-the-deal.rs/src/examples/airtight_disguised.rs:19:7
///    |
/// 19 |     #[sealed(airtight)]
///    |       ------ declaration in trait here
/// # */
/// ```
///
/// ## Cursed
///
/// They might be tempted to keep trying even further (not).
///
/// ```rust ,compile_fail
/// struct Cursed;
///
/// trait Huh<U> {}
/// impl<T, U> Huh<U> for T {}
///
/// impl ::seal_the_deal::examples::Airtight for Cursed {
///     fn generic_method<F: FnOnce() + Huh<impl Sized>>(&self, f: F) -> i32 { // cursed
///         0
///     }
/// }
/// ```
///
/// They'd then get:
///
/// ```rust ,ignore
/// # /*
/// error[E0053]: method `generic_method` has an incompatible type for trait
///   --> src/examples/mod.rs:140:70
///    |
/// 11 |     fn generic_method<F: FnOnce() + Huh<impl Sized>>(&self, f: F) -> i32 { // cursed
///    |                                                                      ^^^
///    |                                                                      |
///    |                                                                      expected associated type, found `i32`
///    |                                                                      help: change the output type to match the trait: `<F as examples::__Airtightඞseal_the_deal_airtight::Sealed<impl Sized>>::ඞRet<i32>`
///    |
///    = note: expected signature `fn(&Cursed, _) -> <F as examples::__Airtightඞseal_the_deal_airtight::Sealed<impl Sized>>::ඞRet<i32>`
///               found signature `fn(&Cursed, _) -> i32`
/// # */
/// ```
///
pub
mod ux_and_error_messages {}
