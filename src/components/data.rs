use dioxus::core::ScopeState;
use dioxus::hooks::{use_ref, UseRef};
use gloo_storage::{LocalStorage, Storage};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::midi::midi_message::MidiMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppData {
    pub presets: Vec<Preset>,
}

impl Default for AppData {
    fn default() -> Self {
        AppData { presets: vec![] }
    }
}

impl AppData {
    pub fn update_preset(self, index: usize, preset: Preset) -> Self {
        let mut presets = self.presets;
        presets[index] = preset;
        AppData { presets, ..self }
    }
    pub fn new_preset(self, preset: Preset) -> Self {
        let mut presets = self.presets;
        presets.push(preset);
        AppData { presets, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub device_index: usize,
    pub label: String,
    pub messages: Vec<MidiMessage>,
    pub card_colour: String,
}

impl Default for Preset {
    fn default() -> Self {
        Preset {
            label: "New Preset".to_string(),
            messages: vec![],
            card_colour: "red".to_string(),
            device_index: 0,
        }
    }
}
/// A persistent storage hook that can be used to store data across application reloads.
#[allow(clippy::needless_return)]
pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static>(
    cx: &ScopeState,
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> &UsePersistent<T> {
    let state = use_ref(cx, move || {
        let key = key.to_string();
        let value = LocalStorage::get(key.as_str()).ok().unwrap_or_else(init);
        StorageEntry { key, value }
    });
    cx.use_hook(|| UsePersistent {
        inner: state.clone(),
    })
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

pub struct UsePersistent<T: 'static> {
    inner: UseRef<StorageEntry<T>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
    /// Returns a reference to the value
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    pub fn set(&self, value: T) {
        let mut inner = self.inner.write();
        let _ = LocalStorage::set(inner.key.as_str(), &value);
        inner.value = value;
    }
}
