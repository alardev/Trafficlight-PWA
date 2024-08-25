use dioxus::prelude::*;
use tracing::{error, info};

use crate::ble::BleService;

// static NAME: GlobalSignal<BleService> = Signal::global(|| 
//     "world".to_string()
// );

pub fn BleSection() -> Element {

    let blepool = match BleService::new() {
        Err(e) => {
            error!("Bluetooth can not be used: {:?}", e);
            None
        }
        Ok(blepool) => Some(blepool),
    };
    
    let mut state = use_signal(|| blepool);
    let is_connected = use_memo(move || state.read().connection.is_some());

    rsx! {
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
                if state.read().is_some {
                    match state.write().try_connect().await {
                        Ok(id) => info!("Success {}", id),
                        Err(e) => error!("Shit! {}", e)
                    }
                    needs_update();
                }
            },
            class: "btn btn-md",
            // disabled: !is_available(),
            "Scan devices"
        }
        button {
            onclick: move |_| async move {
                match state.write().write_green().await {
                    Ok(id) => info!("Success {:?}", id),
                    Err(e) => error!("Error writing to the traffic! {}", e)
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

