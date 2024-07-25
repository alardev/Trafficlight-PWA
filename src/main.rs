#![allow(non_snake_case, dead_code)]

use dioxus::prelude::*;
use tracing::{error, info, Level};
use wasm_bindgen::JsValue;

const PRIMARY_SERVICE_UUID: &str = "6f59f19e-2f39-49de-8525-5d2045f4d999";
const WRITE_CHARACTERISTIC_UUID: &str = "420ece2e-c66c-4059-9ceb-5fc19251e453";
const READ_CHARACTERISTIC_UUID: &str = "a9bf2905-ee69-4baa-8960-4358a9e3a558";

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}


fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let blepool = match BleService::new() {
        Err(e) => {
            error!("Bluetooth can not be used: {:?}", e);
            None
        }
        Ok(blepool) => Some(blepool),
    };

    let mut state = use_signal(|| blepool.unwrap());
    let is_connected = use_memo(move || state.read().connection.is_some());


    
    rsx! {
        div {
            class: "m-auto",
            h1 {
                class: "text-5xl",
                "🚦Trafficlight"
            }
            div {
                class: "flex flex-col 
                justify-center max-w-fit min-w-dvh 
                mx-auto content-center space-y-6 mt-10",
                h2 {
                    class: "flex whitespace-pre",
                    "Connection Status: " 
                    {
                        match is_connected() {
                            true => rsx!(
                                h2 {
                                    class: "text-green-500",
                                    "Connected"
                                }
                            ),
                            false => rsx!(
                                h2 {
                                    class: "text-red-500",
                                    "Disconnected"
                                }
                            ),
                        }
                            
                    }
                }
                button {
                    onclick: move |_| async move {
                        match state.write().try_connect().await {
                            Ok(id) => info!("Success {}", id),
                            Err(e) => error!("Shit! {}", e)
                        }
                        needs_update();
                    },
                    class: "btn btn-md",
                    "Scan devices"
                }
                button {
                    onclick: move |_| async move {
                        match state.write().write_green().await {
                            Ok(id) => info!("Success {:?}", id),
                            Err(e) => error!("Shit! {}", e)
                        }
                        needs_update();
                    },
                    class: "btn btn-md",
                    role: "button",
                    disabled: !is_connected(),
                    "Flash 🟢"
                }
            }
        }
    }
}

pub type DeviceId = String;

#[derive(Debug)]
pub struct NoWebBluetoothSupport;

struct BleConnection {
    name: Option<String>,
    characteristic: web_sys::BluetoothRemoteGattCharacteristic,
}

struct BleService {
    bluetooth: web_sys::Bluetooth,
    connection: Option<BleConnection>,
}

impl BleService {

    pub fn new(
    ) -> Result<Self, NoWebBluetoothSupport>
    {
        let navigator = web_sys::window()
            .expect("This is running inside a web browser")
            .navigator();

        let bluetooth = navigator.bluetooth().ok_or(NoWebBluetoothSupport)?;

        Ok(
            BleService {
                bluetooth,
                connection: None,
            },
        )
    }

    pub async fn try_connect(
        &mut self
    ) -> Result<String, &'static str> {
        use web_sys::{
            BluetoothLeScanFilterInit, BluetoothRemoteGattCharacteristic,
            BluetoothRemoteGattServer, BluetoothRemoteGattService, RequestDeviceOptions,
        };
    
        let device = wasm_bindgen_futures::JsFuture::from(
            self.bluetooth.request_device(
                RequestDeviceOptions::new().filters(
                    &[BluetoothLeScanFilterInit::new().services(
                        &[wasm_bindgen::JsValue::from(PRIMARY_SERVICE_UUID)]
                            .iter()
                            .collect::<js_sys::Array>(),
                    )]
                    .iter()
                    .collect::<js_sys::Array>(),
                ),
            ),
        )
        .await
        .map_err(|_| "No device actually selected")?;
    
        let device: web_sys::BluetoothDevice = device.into();
        info!("New device: {:?} ({:?})", device.name(), device.id());
    
        let server: BluetoothRemoteGattServer = device
            .gatt()
            .ok_or("No GATT found on device")?
            .connect()
            .js2rs()
            .await
            .map_err(|_| "Failed to connect to GATT")?
            .into();
    
        let service: BluetoothRemoteGattService = server
            .get_primary_service_with_str(PRIMARY_SERVICE_UUID)
            .js2rs()
            .await
            .map_err(|_| "No trafficlight service")?
            .into();
    
        let characteristic: BluetoothRemoteGattCharacteristic = service
            .get_characteristic_with_str(WRITE_CHARACTERISTIC_UUID)
            .js2rs()
            .await
            .map_err(|_| "No trafficlight write characteristic")?
            .into();
    
            info!("Device is a Trafficlight.");
    
            let id = device.id();
    
            self.connection = Some(
                BleConnection {
                    characteristic,
                    name: device.name(),
                },
            );

            Ok(id)
    }

    pub async fn write_green(&mut self) -> Result<(), &'static str> {
        // No error handling because this resource returns success anyway (and the success is
        // indicated remotely)
        let connection = &self.connection;

        let mut value: [u8; 2] = *b"31";

        let _result: JsValue  = connection
        .as_ref()
        .unwrap().characteristic
        .write_value_without_response_with_u8_array(&mut value)
        .js2rs()
        .await
        .map_err(|_| "Error writing green LED!")?;

        Ok(())
    }
}

pub trait PromiseExt {
    fn js2rs(self) -> wasm_bindgen_futures::JsFuture;
}

impl PromiseExt for js_sys::Promise {
    fn js2rs(self) -> wasm_bindgen_futures::JsFuture {
        self.into()
    }
}