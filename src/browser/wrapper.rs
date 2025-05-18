use wasm_bindgen::closure::{Closure, WasmClosureFnOnce};

pub(crate) fn spawn_local<F>(future: F)
where
  F: Future<Output = ()> + 'static,
{
  wasm_bindgen_futures::spawn_local(future);
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
  F: 'static + WasmClosureFnOnce<A, R>,
{
  Closure::wrap(fn_once.into_fn_mut())
}
