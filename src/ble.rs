use tracing::info;
use wasm_bindgen::JsValue;

use crate::helpers::PromiseExt;

const PRIMARY_SERVICE_UUID: &str = "6f59f19e-2f39-49de-8525-5d2045f4d999";
const WRITE_CHARACTERISTIC_UUID: &str = "420ece2e-c66c-4059-9ceb-5fc19251e453";
const READ_CHARACTERISTIC_UUID: &str = "a9bf2905-ee69-4baa-8960-4358a9e3a558";

pub type DeviceId = String;

#[derive(Debug)]
pub struct NoWebBluetoothSupport;

pub struct BleConnection {
    name: Option<String>,
    characteristic: web_sys::BluetoothRemoteGattCharacteristic,
}

pub struct BleService {
    pub bluetooth: web_sys::Bluetooth,
    pub connection: Option<BleConnection>,
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
    
            self.connection = Some(
                BleConnection {
                    characteristic,
                    name: device.name(),
                },
            );

            Ok(device.id())
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