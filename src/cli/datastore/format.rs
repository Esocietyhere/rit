use crate::color::Color;
use rbxcloud::rbx::datastore::{
    ListDataStoresResponse, ListEntriesResponse, ListEntryVersionsResponse,
};
use serde_json::Value;

pub fn format_datastore_list_store(response: ListDataStoresResponse) -> String {
    let mut result = String::new();
    for (index, store) in response.datastores.iter().enumerate() {
        let is_last = index == response.datastores.len() - 1;
        result.push_str(&format!(
            "{}\nCreated: {}{}\n",
            Color::yellow().paint(&format!("datastore {}", store.name)),
            store.created_time,
            if is_last { "" } else { "\n" }
        ));
    }
    result
}

pub fn format_datastore_list_entry(response: ListEntriesResponse) -> String {
    let mut result = String::new();
    for (index, entry) in response.keys.iter().enumerate() {
        let is_last = index == response.keys.len() - 1;
        result.push_str(&format!(
            "{}\nScope: {}{}\n",
            Color::yellow().paint(&format!("key {}", entry.key)),
            entry.scope,
            if is_last { "" } else { "\n" }
        ));
    }
    result
}

pub fn format_datastore_list_entry_version(response: ListEntryVersionsResponse) -> String {
    let mut result = String::new();
    for (index, entry) in response.versions.iter().enumerate() {
        let is_last = index == response.versions.len() - 1;
        let status = if entry.deleted {
            Color::red().paint("DELETING")
        } else {
            Color::green().paint("ACTIVE")
        };
        result.push_str(&format!(
            "{} ({})\nLength:  {}\nCreated: {}\n\n    Object Created: {}{}\n",
            Color::yellow().paint(&format!("version {}", entry.version)),
            status,
            entry.content_length,
            entry.created_time,
            entry.object_created_time,
            if is_last { "" } else { "\n" }
        ));
    }
    result
}

pub fn format_json(response: String) -> String {
    let json_object: Value = serde_json::from_str(&response).unwrap();
    serde_json::to_string_pretty(&json_object).unwrap()
}
