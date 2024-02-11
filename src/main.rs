#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::{Routable, Router};
use log::LevelFilter;

use crate::components::{dashboard::DashboardView, preset::PresetView};
use crate::midi::midi_message::{send_midi_messages, MidiMessage};

mod components;
mod midi;
mod routes;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[route("/")]
    DashboardView {},

    #[route("/preset/:id")]
    PresetView { id: usize },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    dioxus_web::launch(App);

    Ok(())
}

fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "container mx-auto h-screen", Router::<Route> {} }
    ))
}

#[component]
fn SendMessages<'a>(cx: Scope<'a>, midi_messages: &'a UseRef<Vec<MidiMessage>>) -> Element {
    cx.render(rsx!(
        button {
            class: "btn btn-warning",
            onclick: move |_| {
                log::info!("Sending messages: {:?}", midi_messages.read().clone());
                send_midi_messages(1, midi_messages.read().clone())
            },
            "Send messages"
        }
    ))
}
