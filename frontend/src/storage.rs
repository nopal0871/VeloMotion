use crate::models::{CustomEffectPlugin, Project};

const PROJECTS_KEY: &str = "velomotion_projects";
const CUSTOM_EFFECTS_KEY: &str = "velomotion_custom_effects";

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

fn load_json<T: serde::de::DeserializeOwned>(key: &str) -> Vec<T> {
    local_storage()
        .and_then(|s| s.get_item(key).ok().flatten())
        .and_then(|raw| serde_json::from_str::<Vec<T>>(&raw).ok())
        .unwrap_or_default()
}

fn save_json<T: serde::Serialize>(key: &str, items: &[T]) {
    if let (Some(storage), Ok(raw)) = (local_storage(), serde_json::to_string(items)) {
        let _ = storage.set_item(key, &raw);
    }
}

pub fn load_projects() -> Vec<Project> {
    load_json(PROJECTS_KEY)
}

pub fn save_projects(projects: &[Project]) {
    save_json(PROJECTS_KEY, projects);
}

pub fn load_custom_effects() -> Vec<CustomEffectPlugin> {
    load_json(CUSTOM_EFFECTS_KEY)
}

pub fn save_custom_effects(effects: &[CustomEffectPlugin]) {
    save_json(CUSTOM_EFFECTS_KEY, effects);
}
