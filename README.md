<img
    src="https://github.com/user-attachments/assets/c47c4c1c-ef21-4b2a-94c4-d37c3b9b8259"
    height="100px"
/>

# 🦭 `::seal-the-deal` 🦭

Attribute to use on the `trait` methods (or associated functions) that you wish to "seal", a.k.a.,
render them `final`.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/seal-the-deal.rs)
[![Latest version](https://img.shields.io/crates/v/seal-the-deal.svg)](
https://crates.io/crates/seal-the-deal)
[![Documentation](https://docs.rs/seal-the-deal/badge.svg)](
https://docs.rs/seal-the-deal)
[![MSRV](https://img.shields.io/badge/MSRV-1.78.0-white)](
https://gist.github.com/danielhenrymantilla/9b59de4db8e5f2467ed008b3c450527b)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/seal-the-deal.svg)](
https://github.com/danielhenrymantilla/seal-the-deal.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/seal-the-deal.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/seal-the-deal.rs/actions)
[![no_std compatible](https://img.shields.io/badge/no__std-compatible-success.svg)](
https://github.com/rust-secure-code/safety-dance/)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

**It shall be impossible for implementors to override the default implementation of that
function.**

# Example

```rust
use ::seal_the_deal::with_seals;

#[with_seals]
trait SomeTrait {
    /// Shall always return `42`.
    #[sealed]
    fn some_method(&self) -> i32 {
        42
    }
}
```

Attempting to override the default impl shall result in a compile error:

```rust ,compile_fail
use ::seal_the_deal::with_seals;

#[with_seals]
trait SomeTrait {
    /// Shall always return `42`.
    #[sealed]
    fn some_method(&self) -> i32 {
        42
    }
}

struct Evil;

impl SomeTrait for Evil {
    fn some_method(&self) -> i32 {
        eprintln!("**manic laughter**");
        27
    }
}
```

with:

```rust ,ignore
# /*
error[E0195]: lifetime parameters or bounds on method `some_method` do not match the trait declaration
  --> src/_lib.rs:61:19
   |
10 |     #[sealed]
   |       ------ lifetimes in impl do not match this method in trait
...
19 |     fn some_method(&self) -> i32 {
   |                   ^ lifetimes do not match method in trait
# */
```

# Implementation a.k.a. the macro secret magic sauce 🧙

<details class="custom"><summary><span class="summary-box"><span>Click to show</span></span></summary>

```rust
use ::seal_the_deal::with_seals;

#[with_seals]
pub trait SomeTrait {
    /// Shall always return `42`.
    #[sealed]
    fn some_method(&self) -> i32 {
        42
    }
}
```

expands to:

```rust
mod __SomeTraitඞseal_the_deal {
    pub trait Seal<'__> {}
    impl Seal<'_> for () {}
}

pub trait SomeTrait {
    /// Shall always return `42`.
    fn some_method<'sealed>(&self) -> i32
    where
        () : __SomeTraitඞseal_the_deal::Seal<'sealed>,
    {
        42
    }
}
```

This approach does effectively and nicely seal that method, preventing it from being overridden:

  - since the clause is trivially implemented for for at least one lifetime (in practice all of
    them), callers are not hindered by it.

      - underspecified lifetime params are fine, they do not cause "inference ambiguity errors";
      - this clause is `dyn`-compatible!
      - since the method stays in the actual trait, it remains visible in the docs;
          - a `() : Sealed<'seal>` will be visible, which is rather low-noise w.r.t. the semantics
            involved.

  - and yet the clause appears to be complex/convoluted enough for Rust not to allow skipping it,
    hence the "lifetime mismatch" when people attempt to do actual impls.

  - technically-speaking, it is possible for a same-module or submodule-thereof to have enough
    visibility of `__SomeTraitඞseal_the_deal::Sealed` to be able to repeat, _verbatim_, the clause.
    This ought to be fine since:

      - there is a `ඞ` in that path!!

      - same-crate code is not really the threat/adversarial model, here, but rather, external code
        (be it for Semver or `unsafe`ty reasons).

      - if this were really deemed a problem, the user could then just further encapsulate the whole
        thing in a helper private `mod`ule:

        ```rust
        pub use paranoid::SomeTrait;
        mod paranoid {
            use super::*;

            #[::seal_the_deal::with_seals]
            pub trait SomeTrait {
                /// Shall always return `42`.
                #[sealed]
                fn some_method(&self) -> i32 {
                    42
                }
            }
        }
        ```

</details>

# Alternative

<details class="custom"><summary><span class="summary-box"><span>Click to show</span></span></summary>

The usual approach to have a sealed/`final` method like that is through a blanket-implemented super
`trait` (a.k.a, the "super extension trait").

```rust
trait Trait : FinalMethodsOfTrait {
    /* non-final methods here */
}

trait FinalMethodsOfTrait {
    /// Shall always return `42`.
    fn method(&self) -> i32 {
        42
    }
}

impl<T : ?Sized + Trait> FinalMethodsOfTrait for T {}
```

But this approach has _two_ problems:

  - documentation-wise, it is ugly: the final `.method()` is no longer discoverable on the page
    for `Trait`.

  - Whilst both properly-`Trait`-bounded generic parameters and `dyn Trait`s do let one very
    ergonomically call `.method()` in a blissfully oblivious-to-the-super-extension-trait way, it
    turns out that _concrete_ implementors do not let one perform such ergonomic calls directly: the
    super extension trait is expected to be in scope for the method call to succeed, totally
    shattering, imho, the illusion and magic of the pattern.

    ```rust ,compile_fail
    mod lib {
        pub trait Trait : FinalMethodsOfTrait {
            /* non-final methods here */
        }

        pub trait FinalMethodsOfTrait {
            /// Shall always return `42`.
            fn method(&self) -> i32 {
                42
            }
        }

        impl<T : ?Sized + Trait> FinalMethodsOfTrait for T {}
    }

    use lib::Trait;

    struct Foo;

    impl Trait for Foo {}

    Foo.method(); // Error, `FinalMethodsOfTrait` not in scope! 😭
    ```

    Error message:

    ```rust ,ignore
    # /*
    error[E0599]: no method named `method` found for struct `Foo` in the current scope
      --> src/_lib.rs:136:5
       |
    10 |         fn method(&self) -> i32 {
       |            ------ the method is available for `Foo` here
    ...
    20 | struct Foo;
       | ---------- method `method` not found for this struct
    ...
    24 | Foo.method(); // Error, `FinalMethodsOfTrait` not in scope! 😭
       |     ^^^^^^ method not found in `Foo`
       |
       = help: items from traits can only be used if the trait is in scope
    help: trait `FinalMethodsOfTrait` which provides `method` is implemented but not in scope; perhaps you want to import it
       |
    2  + use lib::FinalMethodsOfTrait;
       |
    # */
    ```

</details>

# Can `unsafe` code rely on the method never being overridden in any way?

Not for the default [`#[sealed]`][sealed] case. Indeed, the reason code trying to override such a
method currently fails is because the compiler is "not smart enough".

Since the compiler ought to be very much allowed to get smarter, it means this method could end up
not being as foolproof in the future.

For a fully, properly airtight, method-sealing tool, do provide the `airtight` argument to
[`#[sealed]`][sealed]:

```rust ,ignore
#[sealed(airtight)]
```

Like this:

```rust
#[seal_the_deal::with_seals]
pub trait Example {
    #[sealed(airtight)]
    /// This shall always return a pointer valid for reads
    fn valid_pointer() -> *const i32 {
        &42 as &'static i32
    }
}

pub fn sound<T: Example>() -> i32 {
    let ptr: *const i32 = T::valid_pointer();
    unsafe {
        // SAFETY: the default implementation of that function
        // has been sealed in a fully airtight manner.
        *ptr
    }
}
```

This does break `dyn`-compatibility, and also makes the attribute default to hiding its shenanigans
from the rendered documentation.

See the docs of [`#[sealed]`][sealed] for more info about its arguments.

[sealed]: https://docs.rs/seal-the-deal/^0.1/seal_the_deal/attr.sealed.html
