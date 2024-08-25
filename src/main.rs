#![allow(non_snake_case, dead_code)]

mod ble;
mod tg;
mod helpers;
mod components;

use components::ble::BleSection;
use dioxus::prelude::*;
use tracing::{error, info, Level};
use crate::ble::BleService;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
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
                BleSection {}
                button {
                    // onclick: move |_| {
                    // // let value = options.clone();
                    // //     async move {
                    // //         match init_tdclient(value) {
                    // //             Ok(id) => info!("Success {:?}", id.as_string()),
                    // //             Err(e) => error!("Shit! {:?}", e.as_string())
                    // //         }
                    // //         needs_update();
                    // //     }
                    // },
                    class: "btn btn-md",
                    "Initialize Telegram"
                }
            }
        }
    }
}