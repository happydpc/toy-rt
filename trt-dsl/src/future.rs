use futures::{
    future::{LocalBoxFuture, Shared},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct PyFuture<T>(Shared<LocalBoxFuture<'static, T>>);

impl<T: 'static + Clone> PyFuture<T> {
    pub fn new(fut: impl Future<Output = T> + 'static) -> Self {
        Self(fut.boxed_local().shared())
    }

    pub fn shared(&self) -> impl Future<Output = T> {
        self.0.clone()
    }

    pub fn ready(x: T) -> Self {
        Self::new(future::ready(x))
    }

    pub fn map<U: 'static + Clone>(self, f: impl FnOnce(T) -> U + 'static) -> PyFuture<U> {
        PyFuture::new(self.0.map(f))
    }
}
