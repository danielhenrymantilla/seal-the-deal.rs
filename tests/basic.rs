#![allow(async_fn_in_trait)]

use ::seal_the_deal::with_seals;

#[with_seals]
pub trait SomeTrait {
    /// Shall always return `42`.
    #[sealed]
    fn some_method(&self) -> i32 {
        if false {
            return 27;
        }
        42
    }

    #[sealed]
    fn generic_method<F: FnOnce()>(&self, f: F) -> i32 {
        if false {
            return 27;
        }
        f();
        42
    }

    #[sealed]
    async fn async_method() -> u8 {
        if false {
            return 27;
        }
        async {}.await;
        42
    }

    #[sealed]
    async fn async_generic_method<F: FnOnce()>(f: F) -> u8 {
        if false {
            return 27;
        }
        f();
        async {}.await;
        42
    }
}

async fn _call_sites<T: SomeTrait>(it: &T) {
    let _: i32 = it.some_method();
    it.some_method();

    let _: i32 = it.generic_method::<fn()>(|| ());
    it.generic_method::<fn()>(|| ());

    T::async_method().await;
    let _: u8 = T::async_method().await;
}
