use ansi_term::Colour;
use rbxcloud::rbx::datastore::{
    ListDataStoresResponse, ListEntriesResponse, ListEntryVersionsResponse,
};

pub fn format_datastore_store(response: ListDataStoresResponse) -> String {
    let mut result = String::new();
    for (index, store) in response.datastores.iter().enumerate() {
        let is_last = index == response.datastores.len() - 1;
        result.push_str(&format!(
            "{}\nCreated: {}{}\n",
            Colour::Yellow.paint(format!("datastore {}", store.name)),
            store.created_time,
            if is_last { "" } else { "\n" }
        ));
    }
    result
}

pub fn format_datastore_entry(response: ListEntriesResponse) -> String {
    let mut result = String::new();
    for (index, entry) in response.keys.iter().enumerate() {
        let is_last = index == response.keys.len() - 1;
        result.push_str(&format!(
            "{}\nScope: {}{}\n",
            Colour::Yellow.paint(format!("key {}", entry.key)),
            entry.scope,
            if is_last { "" } else { "\n" }
        ));
    }
    result
}

pub fn format_datastore_entry_version(response: ListEntryVersionsResponse) -> String {
    let mut result = String::new();
    for (index, entry) in response.versions.iter().enumerate() {
        let is_last = index == response.versions.len() - 1;
        let status = if entry.deleted {
            Colour::Red.paint("DELETING")
        } else {
            Colour::Green.paint("ACTIVE")
        };
        result.push_str(&format!(
            "{} ({})\nLength:  {}\nCreated: {}\n\n    Object Created: {}{}\n",
            Colour::Yellow.paint(format!("version {}", entry.version)),
            status,
            entry.content_length,
            entry.created_time,
            entry.object_created_time,
            if is_last { "" } else { "\n" }
        ));
    }
    result
}
