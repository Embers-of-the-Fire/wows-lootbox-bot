use std::{marker::PhantomData, pin::Pin};

use futures::Future;

pub trait AsyncClosure {
    type Output;
    type Param;

    fn call(self: Box<Self>, param: Self::Param) -> Pin<Box<dyn Future<Output = Self::Output>>>;
}

pub struct AsyncFnOnce<Fn, Param, Fut, Out>
where
    Fn: FnOnce(Param) -> Fut + Send + 'static,
    Fut: Future<Output = Out> + Send + 'static,
{
    func: Fn,
    _p: PhantomData<(Param, Fut, Out)>,
}

impl<Fn, Param, Fut, Out> AsyncFnOnce<Fn, Param, Fut, Out>
where
    Fn: FnOnce(Param) -> Fut + Send + 'static,
    Fut: Future<Output = Out> + Send + 'static,
{
    pub const fn new(func: Fn) -> Self {
        Self {
            func,
            _p: PhantomData,
        }
    }
}

impl<Fn, Param, Fut, Out> From<Fn> for AsyncFnOnce<Fn, Param, Fut, Out>
where
    Fn: FnOnce(Param) -> Fut + Send + 'static,
    Fut: Future<Output = Out> + Send + 'static,
{
    fn from(value: Fn) -> AsyncFnOnce<Fn, Param, Fut, Out> {
        AsyncFnOnce::new(value)
    }
}

impl<Fn, Param: 'static, Fut, Out: 'static> AsyncClosure for AsyncFnOnce<Fn, Param, Fut, Out>
where
    Fn: FnOnce(Param) -> Fut + Send + 'static,
    Fut: Future<Output = Out> + Send + 'static,
{
    type Output = Out;
    type Param = Param;

    fn call(self: Box<Self>, param: Self::Param) -> Pin<Box<dyn Future<Output = Self::Output>>> {
        Box::pin(async move { (self.func)(param).await })
    }
}
