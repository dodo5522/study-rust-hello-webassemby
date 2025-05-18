pub(crate) fn spawn_local<F>(future: F)
where
  F: Future<Output = ()> + 'static,
{
  wasm_bindgen_futures::spawn_local(future);
}
