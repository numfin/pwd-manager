use std::sync::Arc;

use poll_promise::Promise;

use crate::mutex::Mutex;

#[derive(Debug, Default)]
pub enum LazyValue<T: Send + 'static> {
    #[default]
    Unset,
    Loading,
    Loaded(T),
    Error(eyre::Error),
}

/// This shit helps me stay sane when working with Futures in egui
pub struct LazyLoader<T: Send + 'static> {
    promise: Option<Promise<LazyValue<T>>>,
}
// We need custom default, because compiler talks shit if T doesnt have Default
impl<T: Send + 'static> Default for LazyLoader<T> {
    fn default() -> Self {
        Self { promise: None }
    }
}

impl<T: Send + 'static> LazyLoader<T> {
    /// Use `loader_state` to know promise state between renders
    ///
    /// `loader` is async closure that returns eyre::Result so we have nice and easy async stuff;
    /// ```
    /// let state = Arc::new(Mutex::new(state));
    /// let loader = LazyLoader::load(state.clone(), async {/* ... */});
    /// match loader.check() {
    ///     LazyValue::Unset => /* ... */,
    ///     LazyValue::Loading => /* ... */,
    ///     LazyValue::Loaded(data) => /* ... */,
    ///     LazyValue::Error(err) => /* ... */,
    /// }
    pub fn load_sync<F: std::future::Future<Output = eyre::Result<T>> + 'static + Send>(
        loader_state: Arc<Mutex<LazyLoader<T>>>,
        loader: F,
    ) -> Arc<Mutex<Self>> {
        let _ = {
            loader_state.lock().promise.get_or_insert_with(|| {
                Promise::spawn_async(async {
                    match loader.await {
                        Ok(data) => LazyValue::Loaded(data),
                        Err(err) => LazyValue::Error(err),
                    }
                })
            })
        };
        loader_state
    }
    pub fn load<F: std::future::Future<Output = eyre::Result<T>> + 'static + Send>(
        &mut self,
        loader: F,
    ) -> &mut Self {
        let _ = {
            self.promise.get_or_insert_with(|| {
                Promise::spawn_async(async {
                    match loader.await {
                        Ok(data) => LazyValue::Loaded(data),
                        Err(err) => LazyValue::Error(err),
                    }
                })
            })
        };
        self
    }
    /// Use this to cancel previous promise and replace it with new one
    ///
    /// `loader` is async closure that returns eyre::Result so we have nice and easy async stuff;
    /// ```
    /// let state = Arc::new(Mutex::new(state));
    /// state.lock().update(state.clone(), async {/* ... */});
    pub fn update<F: std::future::Future<Output = eyre::Result<T>> + 'static + Send>(
        &mut self,
        loader: F,
    ) -> &mut Self {
        let _ = {
            self.cancel().promise.insert(Promise::spawn_async(async {
                match loader.await {
                    Ok(data) => LazyValue::Loaded(data),
                    Err(err) => LazyValue::Error(err),
                }
            }))
        };
        self
    }
    pub fn extract_data(&mut self) -> Option<&mut T> {
        self.promise
            .as_mut()
            .and_then(|promise| match promise.ready_mut() {
                Some(lazy_value) => Some(lazy_value),
                _ => None,
            })
            .and_then(|lazy_value| match lazy_value {
                LazyValue::Loaded(data) => Some(data),
                _ => None,
            })
    }
    pub fn cancel(&mut self) -> &mut Self {
        let promise = self.promise.take();
        if let Some(promise) = promise {
            promise.abort();
        }
        self
    }
    pub fn check(&self) -> &LazyValue<T> {
        match &self.promise {
            Some(promise) => match promise.ready() {
                Some(data) => data,
                None => &LazyValue::Loading,
            },
            None => &LazyValue::Unset,
        }
    }
    pub fn is_loading(&self) -> bool {
        match self.check() {
            LazyValue::Loading => true,
            _ => false,
        }
    }
}
