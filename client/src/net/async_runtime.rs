use std::sync::Arc;

use godot::{classes::Engine, prelude::*};
use tokio::{
    runtime::{self, Runtime},
    task::JoinHandle,
};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct AsyncRuntime {
    base: Base<Object>,
    runtime: Arc<Runtime>,
}

#[godot_api]
impl IObject for AsyncRuntime {
    fn init(base: Base<Object>) -> Self {
        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Self {
            base,
            runtime: Arc::new(runtime),
        }
    }
}

#[godot_api]
impl AsyncRuntime {
    pub const SINGLETON: &'static str = "ASYNC_RUNTIME";

    fn singleton() -> Option<Gd<AsyncRuntime>> {
        match Engine::singleton().get_singleton(Self::SINGLETON) {
            Some(sing) => Some(sing.cast::<Self>()),
            None => None,
        }
    }

    pub fn runtime() -> Arc<Runtime> {
        match Self::singleton() {
            Some(s) => {
                let bind = s.bind();
                Arc::clone(&bind.runtime)
            }
            None => {
                Engine::singleton().register_singleton(Self::SINGLETON, &AsyncRuntime::new_alloc());

                let s = Self::singleton().expect("Fatal Error");

                let bind = s.bind();
                Arc::clone(&bind.runtime)
            }
        }
    }

    pub fn spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::runtime().spawn(future)
    }

    pub fn block_on<F>(future: F) -> F::Output
    where
        F: Future,
    {
        Self::runtime().block_on(future)
    }

    pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        Self::runtime().spawn_blocking(func)
    }
}
