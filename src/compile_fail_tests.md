# The following snippets fail to compile

## The main point of the crate

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

enum Evil {}

impl SomeTrait for Evil {
    fn some_method(&self) -> i32 {
        eprintln!("**manic laughter**");
        27
    }
}
```

## Improper usages

```rust ,compile_fail
use ::seal_the_deal::*;

#[with_seals(extraneous)]
trait Trait {}
```

```rust ,compile_fail
use ::seal_the_deal::*;

#[with_seals]
fn not_a_trait() {}
```

```rust ,compile_fail
use ::seal_the_deal::*;

#[with_seals]
trait Trait {
    #[sealed]
    #[sealed]
    fn foo() {}
}
```

```rust ,compile_fail
use ::seal_the_deal::*;

#[with_seals]
trait Trait {
    #[sealed(extraneous)]
    fn foo() {}
}
```

## Sealed method must have a default body

```rust ,compile_fail
use ::seal_the_deal::*;

#[with_seals]
trait Trait {
    #[sealed]
    fn silly();
}
```

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->
