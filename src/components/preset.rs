use std::fmt::Display;

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use gloo_storage::Storage;

use crate::components::data::{use_persistent, AppData, Preset};
use crate::midi::control_change::ControlChange;
use crate::midi::midi_message::{send_midi_messages, MidiMessage};
use crate::midi::program_change::ProgramChange;
use crate::Route;

#[component]
pub fn PresetView(cx: Scope, id: usize) -> Element {
    let data = use_persistent(cx, "app_data", || AppData::default());
    let preset = data.get().presets.get(*id).unwrap().clone();
    log::info!("Loaded Preset: {:?}", preset);

    let messages: &UseRef<Vec<MidiMessage>> = use_ref(cx, || preset.messages.clone());
    let label: &UseState<String> = use_state(cx, || preset.label.clone());
    let colour: &UseState<String> = use_state(cx, || preset.card_colour.to_string());
    let nav = use_navigator(cx);

    cx.render(rsx!(
        div { class: "container bg-slate-50 mx-auto px-2 pt-2 text-sm",
            div {
                div { class: "flex",
                    PresetLabel {
                        value: label,
                        on_change: move |e: String| {
                            log::info!("Updating label: {}", e);
                            label.set(e);
                        }
                    }
                    ColourSelector {
                        current: colour,
                        on_change: move |e: String| {
                            log::info!("Updating colour: {}", e);
                            colour.set(e);
                        }
                    }
                    Button {
                        styling: "flex-1 p-2 rounded-l-md rounded-r-none border-l-2 border-y-2 border-r-0",
                        text: "Save Preset",
                        icon: "fas fa-save",
                        on_click: move |_| {
                            data.set(
                                data
                                    .get()
                                    .update_preset(
                                        *id,
                                        Preset {
                                            label: label.get().clone(),
                                            messages: messages.read().clone(),
                                            card_colour: colour.get().to_string(),
                                        },
                                    ),
                            );
                        }
                    }
                    Button {
                        styling: "flex-1 p-2 rounded-r-md rounded-l-none border-r-2 border-y-2 border-l--0",
                        text: "Test Messages",
                        icon: "fas fa-paper-plane",
                        on_click: move |_| { send_midi_messages(data.get().device_index, messages.read().clone()) }
                    }
                    Button {
                        styling: "flex p-2 rounded-lg border-2 ml-2",
                        text: "Delete Preset",
                        icon: "fas fa-trash",
                        on_click: move |_| {
                            let mut presets = data.get().presets.clone();
                            presets.remove(*id);
                            data.set(AppData { presets, ..data.get() });
                            nav.replace(Route::DashboardView {});
                        }
                    }
                }
            }
            div { class: "flex mt-2",
                div {
                    Button {
                        styling: "bg-teal-300 hover:bg-teal-500 border-teal-500",
                        text: "Add Message",
                        icon: "fas fa-plus",
                        on_click: move |_| {
                            messages.with_mut(|vec| vec.push(MidiMessage::default()));
                        }
                    }
                }
                div {
                    Button {
                        styling: "bg-red-300 hover:bg-red-500 border-red-500",
                        text: "Remove Message",
                        icon: "fas fa-minus",
                        on_click: move |_| {
                            messages
                                .with_mut(|vec| {
                                    if vec.len() > 0 {
                                        vec.pop();
                                    }
                                });
                        }
                    }
                }
            }
            div {
                for (i , message) in messages.read().iter().enumerate() {
                    MessageView {
                        index: i,
                        current_value: message.clone(),
                        on_change: move |e: MidiMessage| {
                            log::info!("Updating message {}: {:?}", i, e);
                            messages
                                .with_mut(|vec| {
                                    vec[i] = e.clone();
                                });
                        }
                    }
                }
            }
        }
    ))
}

#[component]
fn ColourSelector<'a>(
    cx: Scope<'a>,
    current: &'a UseState<String>,
    on_change: EventHandler<'a, String>,
) -> Element {
    let tailwind_colours = vec![
        "gray", "slate", "zinc", "neutral", "stone", "red", "orange", "amber", "yellow", "lime",
        "green", "emerald", "teal", "cyan", "sky", "blue", "indigo", "violet", "purple", "fuchsia",
        "pink", "rose",
    ];
    cx.render(rsx!(
        div { class: "flex-1",
            select {
                class: "flex rounded-md border-2 px-5 py-2",
                onchange: move |e| on_change.call(e.value.to_string()),
                for colour in tailwind_colours.into_iter() {
                    option { selected: current.get() == &colour.to_string(), colour.to_string() }
                }
            }
        }
    ))
}

#[derive(Debug, Clone, PartialEq)]
enum MessageType {
    ProgramChange,
    ControlChange,
    Empty,
}

impl MessageType {
    fn all() -> Vec<MessageType> {
        vec![
            MessageType::Empty,
            MessageType::ProgramChange,
            MessageType::ControlChange,
        ]
    }
}

impl From<&MessageType> for MidiMessage {
    fn from(message_type: &MessageType) -> MidiMessage {
        match message_type {
            MessageType::ProgramChange => MidiMessage::ProgramChange(ProgramChange::default()),
            MessageType::ControlChange => MidiMessage::ControlChange(ControlChange::default()),
            MessageType::Empty => MidiMessage::Empty,
        }
    }
}

impl From<&str> for MessageType {
    fn from(message: &str) -> Self {
        match message {
            "Program Change" => MessageType::ProgramChange,
            "Control Change" => MessageType::ControlChange,
            _ => MessageType::Empty,
        }
    }
}

impl From<MidiMessage> for MessageType {
    fn from(message: MidiMessage) -> Self {
        match message {
            MidiMessage::ProgramChange(_) => MessageType::ProgramChange,
            MidiMessage::ControlChange(_) => MessageType::ControlChange,
            _ => MessageType::Empty,
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::ProgramChange => write!(f, "Program Change"),
            MessageType::ControlChange => write!(f, "Control Change"),
            MessageType::Empty => write!(f, "Empty"),
        }
    }
}

#[component]
pub fn Button<'a>(
    cx: Scope<'a>,
    styling: &'a str,
    text: &'a str,
    icon: &'a str,
    on_click: EventHandler<'a>,
) -> Element {
    cx.render(rsx!(
        button {
            class: "border-2 rounded-md flex text-sm items-center px-2 py-2 hover:bg-slate-200 md:px-5 {styling}",
            onclick: move |e| on_click.call(()),
            span { class: "{icon} mr-1" }
            text.to_string()
        }
    ))
}

#[component]
fn SelectBox<'a>(cx: Scope<'a>, selected: &'a str, choices: Vec<&'a str>) -> Element {
    let selected = use_state(cx, || selected.to_string());
    cx.render(rsx!(
        div {
            span { class: "border-2 rounded-l-md text-sm flex items-center px-5", "Type" }
            input {
                r#type: "text",
                id: "website-admin",
                class: "rounded-r-md border block items-center text-sm p-3",
                placeholder: "elonmusk"
            }
        }
    ))
}

#[component]
fn PresetLabel<'a>(
    cx: Scope<'a>,
    value: &'a UseState<String>,
    on_change: EventHandler<'a, String>,
) -> Element {
    cx.render(rsx!(
        input {
            class: "flex-auto border-2 text-sm rounded-md p-2 mr-2",
            r#type: "text",
            placeholder: "Preset Label",
            onchange: move |e| on_change.call(e.value.to_string()),
            value: "{value}"
        }
    ))
}

#[component]
fn SavePresetButton(cx: Scope) -> Element {
    cx.render(rsx!(
        button { class: "flex-1 p-2 rounded-l-md border-l-2 border-y-2 hover:bg-slate-200",
            "Save Preset"
        }
    ))
}

#[component]
fn SendMessageButton(cx: Scope) -> Element {
    cx.render(rsx!(
        button { class: "flex-1 p-2 border-2 rounded-r-md hover:bg-slate-200", "Test Messages" }
    ))
}

#[component]
fn MessageView<'a>(
    cx: Scope<'a>,
    index: usize,
    current_value: MidiMessage,
    on_change: EventHandler<'a, MidiMessage>,
) -> Element {
    let message_type: &UseState<MessageType> = use_state(cx, || current_value.clone().into());
    cx.render(rsx!(
        div { class: "py-1",
            div { class: "flex",
                MessageIndex { index: *index }
                MessageType {
                    current: message_type,
                    on_change: move |e: Event<FormData>| {
                        let mt = MessageType::from(e.value.as_str());
                        message_type.set(mt.clone());
                        on_change.call(MidiMessage::from(&mt));
                    }
                }
                ClearMessageButton {
                    on_click: move |_| {
                        message_type.set(MessageType::Empty);
                        on_change.call(MidiMessage::Empty);
                    }
                }
            }
            div { class: "mx-4",
                match current_value {
                    MidiMessage::ProgramChange(pc) => rsx!(ProgramChangeView {
                        current_value: pc.clone(),
                        on_change: move |pc: ProgramChange| on_change.call(MidiMessage::ProgramChange(pc))
                    }),
                    MidiMessage::ControlChange(cc) => rsx!(ControlChangeView {
                        current_value: cc.clone(),
                        on_change: move |cc: ControlChange| on_change.call(MidiMessage::ControlChange(cc))
                    }),
                    _ => rsx!({}),
                }
            }
        }
    ))
}

#[component]
fn ClearMessageButton<'a>(cx: Scope<'a>, on_click: EventHandler<'a>) -> Element {
    cx.render(rsx!(
        button {
            class: "flex-auto border-2 rounded-md items-center px-2 transition hover:bg-slate-200 md:px-5",
            onclick: move |e| on_click.call(()),
            span { class: "fas fa-trash pr-2" }
            "Clear message"
        }
    ))
}

#[component]
fn MessageIndex(cx: Scope, index: usize) -> Element {
    cx.render(rsx!(
        span { class: "border-2 rounded-md flex items-center p-2 font-mono bg-grey-300", "{index}" }
    ))
}

#[component]
fn MessageType<'a>(
    cx: Scope<'a>,
    current: &'a UseState<MessageType>,
    on_change: EventHandler<'a, Event<FormData>>,
) -> Element {
    cx.render(rsx!(
        div { class: "flex mx-1 md:mx-1",
            span { class: "flex rounded-l-md border-2 items-center px-2 py-2 md:px-5", "Type" }
            select {
                class: "flex rounded-r-md border-y-2 border-r-2 items-center px-5 py-2",
                onchange: move |e| on_change.call(e),
                for mt in MessageType::all().into_iter() {
                    option { selected: current.get() == &mt, mt.to_string() }
                }
            }
        }
    ))
}

#[component]
fn ProgramChangeView<'a>(
    cx: Scope<'a>,
    current_value: ProgramChange,
    on_change: EventHandler<'a, ProgramChange>,
) -> Element {
    cx.render(rsx!(
        div { class: "flex my-1 p-1 border-2 rounded-md border-l-4 border-l-indigo-500 bg-grey-100",
            NumberView {
                min_value: 1,
                max_value: 16,
                current_value: current_value.channel as i32,
                label: "MIDI Channel".to_string(),
                on_change: move |i: i32| {
                    on_change
                        .call(ProgramChange {
                            channel: i as u8,
                            program: current_value.program,
                        })
                }
            }
            NumberView {
                min_value: 0,
                max_value: 127,
                current_value: current_value.program as i32,
                label: "PC Number".to_string(),
                on_change: move |i: i32| {
                    on_change
                        .call(ProgramChange {
                            channel: current_value.channel,
                            program: i as u8,
                        })
                }
            }
        }
    ))
}

#[component]
fn Badge(cx: Scope, text: String) -> Element {
    cx.render(rsx!(
        span { class: "inline-flex text-nowrap min-w-full items-center rounded-md bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600 ring-1 ring-inset ring-gray-500/10",
            text.to_string()
        }
    ))
}
#[component]
fn NumberView<'a>(
    cx: Scope,
    min_value: i32,
    max_value: i32,
    label: String,
    current_value: i32,
    on_change: EventHandler<'a, i32>,
) -> Element {
    let possible_numbers = *min_value..=*max_value;
    cx.render(rsx!(
        div { class: "relative",
            span { class: "absolute pl-3", Badge { text: label.to_string() } }
        }
        select {
            id: "number-{label}",
            class: "peer flex-1 rounded-md border-2 px-5 py-2 mx-2 mt-4 md:px-5 text-end align-text-bottom",
            onchange: move |e| on_change.call(e.value.parse::<i32>().unwrap()),
            for nr in possible_numbers {
                option { selected: current_value == &nr, "{nr}" }
            }
        }
    ))
}

#[component]
fn ControlChangeView<'a>(
    cx: Scope,
    current_value: ControlChange,
    on_change: EventHandler<'a, ControlChange>,
) -> Element {
    cx.render(rsx!(
        div { class: "flex my-1 p-1 border-2 rounded-md border-l-4 border-l-rose-500 bg-grey-100",
            NumberView {
                min_value: 1,
                max_value: 16,
                current_value: current_value.channel as i32,
                label: "MIDI Channel".to_string(),
                on_change: move |i: i32| {
                    on_change
                        .call(ControlChange {
                            channel: i as u8,
                            control_number: current_value.control_number,
                            value: current_value.value,
                        })
                }
            }
            NumberView {
                min_value: 0,
                max_value: 127,
                current_value: current_value.control_number as i32,
                label: "CC Number".to_string(),
                on_change: move |i: i32| {
                    on_change
                        .call(ControlChange {
                            channel: current_value.channel,
                            control_number: i as u8,
                            value: current_value.value,
                        })
                }
            }
            NumberView {
                min_value: 0,
                max_value: 127,
                current_value: current_value.value as i32,
                label: "CC Value".to_string(),
                on_change: move |i: i32| {
                    on_change
                        .call(ControlChange {
                            channel: current_value.channel,
                            control_number: current_value.control_number,
                            value: i as u8,
                        })
                }
            }
        }
    ))
}
