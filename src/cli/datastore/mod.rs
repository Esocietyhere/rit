mod format;

use super::getenv;
use crate::config::Config;
use clap::{Args, Subcommand, ValueEnum};

use std::io::{stdin, stdout, Write};

use format::{
    format_datastore_get_entry, format_datastore_list_entry, format_datastore_list_entry_version,
    format_datastore_list_store,
};

use rbxcloud::rbx::{
    DataStoreDeleteEntry, DataStoreGetEntry, DataStoreGetEntryVersion, DataStoreIncrementEntry,
    DataStoreListEntries, DataStoreListEntryVersions, DataStoreListStores, DataStoreSetEntry,
    RbxCloud, ReturnLimit, RobloxUserId, UniverseId,
};

#[derive(Debug, Subcommand)]
pub enum DataStoreCommands {
    /// List all DataStores in a given universe
    ListStores {
        /// Return only DataStores with this prefix
        #[clap(short, long, value_parser)]
        prefix: Option<String>,

        /// Maximum number of items to return
        #[clap(short, long, value_parser)]
        limit: Option<u64>,

        /// Cursor for the next set of data
        #[clap(short, long, value_parser)]
        cursor: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },

    /// List all entries in a DataStore
    List {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// If true, return keys from all scopes
        #[clap(short = 'o', long, value_parser)]
        all_scopes: bool,

        /// Return only DataStores with this prefix
        #[clap(short, long, value_parser)]
        prefix: Option<String>,

        /// Maximum number of items to return
        #[clap(short, long, value_parser)]
        limit: Option<u64>,

        /// Cursor for the next set of data
        #[clap(short, long, value_parser)]
        cursor: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },

    /// Get a DataStore entry
    Get {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },

    /// Set or create the value of a DataStore entry
    Set {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// Only update if the current version matches this
        #[clap(short = 'i', long, value_parser)]
        match_version: Option<String>,

        /// Only create the entry if it does not exist
        #[clap(short, long, value_parser)]
        exclusive_create: Option<bool>,

        /// JSON-stringified data (up to 4MB)
        #[clap(short = 'D', long, value_parser)]
        data: String,

        /// Associated UserID (can be multiple)
        #[clap(short = 'U', long, value_parser)]
        user_ids: Option<Vec<u64>>,

        /// JSON-stringified attributes data
        #[clap(short = 't', long, value_parser)]
        attributes: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },

    /// Increment or create the value of a DataStore entry
    Increment {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// The amount by which the entry should be incremented
        #[clap(short, long, value_parser)]
        increment_by: f64,

        /// Comma-separated list of Roblox user IDs
        #[clap(short = 'U', long, value_parser)]
        user_ids: Option<Vec<u64>>,

        /// JSON-stringified attributes data
        #[clap(short = 't', long, value_parser)]
        attributes: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },

    /// Delete a DataStore entry
    Delete {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },

    /// List all versions of a DataStore entry
    ListVersions {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// Start time constraint (ISO UTC Datetime)
        #[clap(short = 't', long, value_parser)]
        start_time: Option<String>,

        /// End time constraint (ISO UTC Datetime)
        #[clap(short = 'e', long, value_parser)]
        end_time: Option<String>,

        /// Sort order
        #[clap(short = 'o', long, value_enum)]
        sort_order: Option<ListEntrySortOrder>,

        /// Maximum number of items to return
        #[clap(short, long, value_parser)]
        limit: Option<u64>,

        /// Cursor for the next set of data
        #[clap(short, long, value_parser)]
        cursor: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },

    /// Get the value of a specific entry version
    GetVersion {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// The version of the key
        #[clap(short = 'i', long, value_parser)]
        version_id: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ListEntrySortOrder {
    Ascending,
    Descending,
}

/// Manage the datastore
#[derive(Debug, Args)]
pub struct DataStore {
    #[clap(subcommand)]
    command: DataStoreCommands,
}

#[inline]
fn u64_ids_to_roblox_ids(user_ids: Option<Vec<u64>>) -> Option<Vec<RobloxUserId>> {
    user_ids.map(|ids| {
        ids.into_iter()
            .map(RobloxUserId)
            .collect::<Vec<RobloxUserId>>()
    })
}

fn universe_id() -> UniverseId {
    UniverseId(Config::new("main".to_string()).get_universe_id().unwrap())
}

impl DataStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            DataStoreCommands::ListStores {
                prefix,
                limit,
                cursor,
                api_key,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();

                let mut has_cursor = true;
                let mut next_cursor = cursor.clone();

                while has_cursor {
                    let res = datastore
                        .list_stores(&DataStoreListStores {
                            cursor: next_cursor,
                            limit: ReturnLimit(limit.unwrap_or(100)),
                            prefix: prefix.clone(),
                        })
                        .await;
                    match res {
                        Ok(data) => {
                            has_cursor = data.next_page_cursor.clone() != Some("".to_string());
                            next_cursor = data.next_page_cursor.clone();
                            println!("{}", format_datastore_list_store(data));
                        }
                        Err(err) => return Err(err.into()),
                    }
                    if !has_cursor {
                        break;
                    }
                    print!("Press Enter to continue or 'q' to quit: ");
                    let _ = stdout().flush();
                    let mut input = String::new();
                    let _ = stdin().read_line(&mut input);

                    match input.trim() {
                        "" => {
                            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        }
                        "q" => break,
                        _ => println!("Invalid input, quitting..."),
                    }
                }
                Ok(None)
            }

            DataStoreCommands::List {
                prefix,
                limit,
                cursor,
                api_key,
                datastore_name,
                scope,
                all_scopes,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();

                let mut has_cursor = true;
                let mut next_cursor = cursor.clone();

                while has_cursor {
                    let res = datastore
                        .list_entries(&DataStoreListEntries {
                            name: datastore_name.clone(),
                            scope: scope.clone(),
                            all_scopes,
                            cursor: next_cursor,
                            limit: ReturnLimit(limit.unwrap_or(100)),
                            prefix: prefix.clone(),
                        })
                        .await;
                    match res {
                        Ok(data) => {
                            has_cursor = data.next_page_cursor.clone() != Some("".to_string());
                            next_cursor = data.next_page_cursor.clone();
                            println!("{}", format_datastore_list_entry(data));
                        }
                        Err(err) => return Err(err.into()),
                    }
                    if !has_cursor {
                        break;
                    }
                    print!("Press Enter to continue or 'q' to quit: ");
                    let _ = stdout().flush();
                    let mut input = String::new();
                    let _ = stdin().read_line(&mut input);

                    match input.trim() {
                        "" => {
                            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        }
                        "q" => break,
                        _ => println!("Invalid input, quitting..."),
                    }
                }
                Ok(None)
            }

            DataStoreCommands::Get {
                datastore_name,
                scope,
                key,
                api_key,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();
                let res = datastore
                    .get_entry_string(&DataStoreGetEntry {
                        name: datastore_name,
                        scope,
                        key,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format_datastore_get_entry(data))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Set {
                datastore_name,
                scope,
                key,
                match_version,
                exclusive_create,
                data,
                user_ids,
                attributes,
                api_key,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();
                let ids = u64_ids_to_roblox_ids(user_ids);
                let res = datastore
                    .set_entry(&DataStoreSetEntry {
                        name: datastore_name,
                        scope,
                        key,
                        match_version,
                        exclusive_create,
                        roblox_entry_user_ids: ids,
                        roblox_entry_attributes: attributes,
                        data,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Increment {
                datastore_name,
                scope,
                key,
                increment_by,
                user_ids,
                attributes,
                api_key,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();
                let ids = u64_ids_to_roblox_ids(user_ids);
                let res = datastore
                    .increment_entry(&DataStoreIncrementEntry {
                        name: datastore_name,
                        scope,
                        key,
                        roblox_entry_user_ids: ids,
                        roblox_entry_attributes: attributes,
                        increment_by,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data}"))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Delete {
                datastore_name,
                scope,
                key,
                api_key,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();
                let res = datastore
                    .delete_entry(&DataStoreDeleteEntry {
                        name: datastore_name,
                        scope,
                        key,
                    })
                    .await;
                match res {
                    Ok(_) => Ok(None),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::ListVersions {
                datastore_name,
                scope,
                key,
                start_time,
                end_time,
                sort_order,
                limit,
                cursor,
                api_key,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();

                let mut has_cursor = true;
                let mut next_cursor = cursor.clone();
                let order = sort_order.unwrap_or(ListEntrySortOrder::Ascending);

                while has_cursor {
                    let res = datastore
                        .list_entry_versions(&DataStoreListEntryVersions {
                            name: datastore_name.clone(),
                            scope: scope.clone(),
                            key: key.clone(),
                            start_time: start_time.clone(),
                            end_time: end_time.clone(),
                            sort_order: format!("{order:?}"),
                            limit: ReturnLimit(limit.unwrap_or(100)),
                            cursor: next_cursor,
                        })
                        .await;
                    match res {
                        Ok(data) => {
                            has_cursor = data.next_page_cursor.clone() != Some("".to_string());
                            next_cursor = data.next_page_cursor.clone();
                            println!("{}", format_datastore_list_entry_version(data));
                        }
                        Err(err) => return Err(err.into()),
                    }
                    if !has_cursor {
                        break;
                    }
                    print!("Press Enter to continue or 'q' to quit: ");
                    let _ = stdout().flush();
                    let mut input = String::new();
                    let _ = stdin().read_line(&mut input);

                    match input.trim() {
                        "" => {
                            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        }
                        "q" => break,
                        _ => println!("Invalid input, quitting..."),
                    }
                }
                Ok(None)
            }

            DataStoreCommands::GetVersion {
                datastore_name,
                scope,
                key,
                version_id,
                api_key,
            } => {
                let rbx_cloud =
                    RbxCloud::new(&getenv(api_key, "OPENCLOUD_KEY".to_string()), universe_id());
                let datastore = rbx_cloud.datastore();
                let res = datastore
                    .get_entry_version(&DataStoreGetEntryVersion {
                        name: datastore_name,
                        scope,
                        key,
                        version_id,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(data)),
                    Err(err) => Err(err.into()),
                }
            }
        }
    }
}
