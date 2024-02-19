use napi::{bindgen_prelude::{Reference, SharedReference}, Env};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct A;

pub struct UsesA<'a> {
    #[allow(unused)]
    used: &'a A,
}

#[napi]
pub struct B {
    #[allow(unused)]
    inner: SharedReference<A, UsesA<'static>>,
}

#[napi]
impl A {
    #[napi(factory)]
    pub fn new() -> Self {
        Self {}
    }

    #[napi]
    pub fn new_b(&self, reference: Reference<A>, env: Env) -> napi::Result<B> {
        Ok(B {
            inner: reference.share_with(env, |a| {
                Ok(UsesA { used: a })
            })?,
        })
    }
}