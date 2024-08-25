use dioxus::hooks::use_signal;
use js_sys::Object;
use serde::{Deserialize, Serialize};
use tracing::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
const tdlib = new tdweb.default({ 
    useTestDC: false,
    readOnly: false,
    verbosity: 3,
    jsVerbosity: 3,
    fastUpdating: true,
    useDatabase: false,
    mode: 'wasm'
 });
 
export { tdlib as TdClient };

")]
extern "C" {
    #[wasm_bindgen]
    pub type TdClient;

    #[wasm_bindgen(js_namespace = ["event"], catch)]
    pub async fn listen(
        event_name: &str,
        cb: &Closure<dyn Fn(JsValue)>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(static_method_of = TdClient, catch)]
    pub fn send(options: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(static_method_of = TdClient, catch)]
    pub fn onUpdate(options: &JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
pub struct TelegramOptions {
    #[serde(rename = "@type")]
    pub attype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TdParameters>,
}

#[derive(Serialize, Deserialize)]
pub struct TdParameters {
    #[serde(rename = "@type")]
    pub attype: String,
    pub use_test_dc: bool,
    pub api_id: String,
    pub api_hash: String,
    pub system_language_code: String,
    pub device_model: String,
    pub application_version: String,
    pub use_secret_chats: bool,
    pub use_message_database: bool,
    pub use_file_database: bool,
    pub files_directory: String,
}


// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen (extends = Event, js_name = Event)]
//     pub type onUpdate;
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = Event)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type JsEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> JsEvent;

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn event_type(this: &JsEvent) -> String;

    #[wasm_bindgen(method, setter, js_name = "type")]
    pub fn set_event_type(this: &JsEvent, value: &str);

    #[wasm_bindgen(method, getter, js_name = target)]
    pub fn target(this: &JsEvent) -> Object;

    #[wasm_bindgen(method, setter, js_name = target)]
    pub fn set_target(this: &JsEvent, value: &Object);

    #[wasm_bindgen(method, getter, js_name = sourceTarget)]
    pub fn source_target(this: &JsEvent) -> Object;

    #[wasm_bindgen(method, setter, js_name = sourceTarget)]
    pub fn set_source_target(this: &JsEvent, value: &Object);

    #[wasm_bindgen(method, getter, js_name = propagatedFrom)]
    pub fn propagated_from(this: &JsEvent) -> Object;

    #[wasm_bindgen(method, setter, js_name = propagatedFrom)]
    pub fn set_propagated_from(this: &JsEvent, value: &Object);

    #[wasm_bindgen(method, getter, js_name = layer)]
    pub fn layer(this: &JsEvent) -> Object;

    #[wasm_bindgen(method, setter, js_name = layer)]
    pub fn set_layer(this: &JsEvent, value: &Object);
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut()
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

// When the Interval is destroyed, clear its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.token);
    }
}

// Keep logging "hello" every second until the resulting `Interval` is dropped.
#[wasm_bindgen]
pub fn hello() -> Interval {
    Interval::new(208_000, || log("hello"))
}

pub async fn init_tdclient() {
    // let on_update = |input: &_| {
    //     info!("well well well");
    // };

    // let closure = Closure::wrap(Box::new(on_update) as Box<dyn Fn(&TdClient) -> ()>);

    let options = TelegramOptions {
        attype: "setTdlibParameters".to_string(),
        parameters: Some(TdParameters {
            attype: "tdParameters".to_string(),
            use_test_dc: false,
            api_id: "10224218".to_string(),
            api_hash: "ea2f45aae6eafd51508609ca4dc34bab".to_string(),
            system_language_code: "en".to_string(),
            device_model: "Rust Dioxus PWA".to_string(),
            application_version: "0.1".to_string(),
            use_secret_chats: false,
            use_message_database: true,
            use_file_database: true,
            files_directory: "/".to_string(),
        })
    };

    // let check_database_key = TelegramOptions {
    //     attype: "checkDatabaseEncryptionKey".to_string(),
    //     parameters: None
    // };

    let thing = &serde_wasm_bindgen::to_value(&options).unwrap();

    let tg = TdClient::send(&thing);
    // match tg {
    //     Ok(sth) => info!("SUCCESS {:?}", sth),
    //     Err(e) => info!("FAIL {:?}", e),
    // }

    let mut tg_svc = use_signal(|| tg.unwrap());

    // let onupd = js_sys::Object::new();
    // js_sys::Reflect::set(&onupd, &JsValue::from("onUpdate"), closure.as_ref())
    //     .expect("Unable to set TdClient::onUpdate()");
    // closure.forget();
}