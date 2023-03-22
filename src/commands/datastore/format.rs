use ansi_term::Colour;
use rbxcloud::rbx::{
    datastore::{ListDataStoresResponse, ListEntriesResponse, ListEntryVersionsResponse},
};

pub fn format_datastore_store(response: ListDataStoresResponse) -> String {
    let mut result = String::new();
    for store in response.datastores {
        result.push_str(&format!(
            "{}\nCreated: {}\n\n",
            Colour::Yellow.paint(format!("datastore {}", store.name)),
            store.created_time
        ));
    };
    result
}

pub fn format_datastore_entry(response: ListEntriesResponse) -> String {
    let mut result = String::new();
    for entry in response.keys {
        result.push_str(&format!(
            "{}\nScope: {}\n\n",
            Colour::Yellow.paint(format!("key {}", entry.key)),
            entry.scope
        ));
    };
    result
}

pub fn format_datastore_entry_version(response: ListEntryVersionsResponse) -> String {
    let mut result = String::new();
    for entry in response.versions {
        let status = if entry.deleted { Colour::Red.paint("DELETING") } else { Colour::Green.paint("ACTIVE") };
        result.push_str(&format!(
            "{} ({})\nLength:  {}\nCreated: {}\n\n    Object Created: {}\n\n",
            Colour::Yellow.paint(format!("version {}", entry.version)),
            status,
            entry.content_length,
            entry.created_time,
            entry.object_created_time
        ));
    };
    result
}