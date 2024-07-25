pub trait PromiseExt {
    fn js2rs(self) -> wasm_bindgen_futures::JsFuture;
}

impl PromiseExt for js_sys::Promise {
    fn js2rs(self) -> wasm_bindgen_futures::JsFuture {
        self.into()
    }
}