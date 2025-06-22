use wasm_bindgen::closure::{Closure, WasmClosure, WasmClosureFnOnce};

use crate::game::engine::browser::LoopClosure;

pub fn spawn_local<F>(future: F)
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

pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
  Closure::new(f)
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
  Closure::wrap(data)
}
