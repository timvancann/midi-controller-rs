use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::data::{use_persistent, AppData};
use crate::components::preset::Button;
use crate::midi::midi_message::send_midi_messages;
use crate::Route;

#[component]
pub fn DashboardView(cx: Scope) -> Element {
    let data = use_persistent(cx, "app_data", || AppData::default());
    let in_edit_mode = use_state(cx, || false);

    cx.render(rsx!(
        div { class: "container bg-slate-50 mx-auto px-2 pt-2 text-sm mb-4",
            div { class: "flex",
                Button {
                    text: "New Preset",
                    icon: "fas fa-plus",
                    styling: "",
                    on_click: move |_| {
                        data.set(data.get().new_preset(Default::default()));
                    }
                }
                Button {
                    text: "Toggle Edit Mode",
                    icon: { if *in_edit_mode.get() { "fas fa-pen-to-square" } else { "fa fa-pen-to-square" } },
                    styling: {
                        if *in_edit_mode.get() {
                            "bg-gray-400 hover:bg-gray-500 border-gray-600"
                        } else {
                            "bg-gray-200 hover:bg-gray-300 border-gray-400"
                        }
                    },
                    on_click: move |_| {
                        in_edit_mode.set(!*in_edit_mode.get());
                    }
                }
                div { class: "ml-4",
                    if *in_edit_mode.get() {
                        rsx! (div { class: "text-center text-gray-600 mt-2",
                            "Edit mode is enabled. Click on a preset to edit it."
                        })
                    } else {
                        rsx!(div { class: "text-center text-gray-600 mt-2",
                            "Edit mode is disabled. Click on a preset to view it."
                        })
                    }
                }
            }
            div { class: "grid grid-cols-2 gap-2 md:grid-cols-3 md:gap-4 mt-4",
                for (i , preset) in data.get().presets.into_iter().enumerate() {
                    if !*in_edit_mode.get() {
                        rsx! (
                    div {
                        class: "block max-w-sm p-6 border-2 border-{preset.card_colour}-700 shadow-lg rounded-lg bg-{preset.card_colour}-300 hover:bg-{preset.card_colour}-500 transition duration-200 ease-in-out hover:cursor-pointer",
                                onclick: move |_| {send_midi_messages(data.get().device_index, preset.messages.clone())},
                        h5 { class: "mb-2 text-2xl font-bold text-center tracking-tight text-gray-900",
                            "{preset.label}"
                        }
                    })
                        }
                    else {
                    rsx!(
                    Link {
                        to: Route::PresetView { id: i },
                        class: "block max-w-sm p-6 border-2 border-{preset.card_colour}-700 shadow-lg rounded-lg bg-{preset.card_colour}-300 hover:bg-{preset.card_colour}-500 transition duration-200 ease-in-out",
                        h5 { class: "mb-2 text-2xl font-bold text-center tracking-tight text-gray-900",
                            "{preset.label}"
                        }
                                span { class: "fas fa-pen"}
                    })
                    }
                }
            }
        }
    ))
}
