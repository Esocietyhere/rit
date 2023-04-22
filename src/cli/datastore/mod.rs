mod format;

use super::getenv;
use crate::config::Config;
use clap::{Args, Subcommand, ValueEnum};

use std::io::{stdin, stdout, Write};

use format::{
    format_datastore_list_entry, format_datastore_list_entry_version, format_datastore_list_store,
    format_json,
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
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

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
        #[clap(short, long, value_parser)]
        api_key: Option<String>,
    },

    /// List all entries in a DataStore
    List {
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: Option<String>,

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
        #[clap(short, long, value_parser)]
        api_key: Option<String>,
    },

    /// Get a DataStore entry
    Get {
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: Option<String>,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: Option<String>,
    },

    /// Set or create the value of a DataStore entry
    Set {
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: Option<String>,

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
        #[clap(short, long, value_parser)]
        api_key: Option<String>,
    },

    /// Increment or create the value of a DataStore entry
    Increment {
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: Option<String>,

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
        #[clap(short, long, value_parser)]
        api_key: Option<String>,
    },

    /// Delete a DataStore entry
    Delete {
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: Option<String>,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The key of the entry
        #[clap(short, long, value_parser)]
        key: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: Option<String>,
    },

    /// List all versions of a DataStore entry
    ListVersions {
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: Option<String>,

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
        #[clap(short, long, value_parser)]
        api_key: Option<String>,
    },

    /// Get the value of a specific entry version
    GetVersion {
        /// The branch to read from
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,

        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: Option<String>,

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
        #[clap(short, long, value_parser)]
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

fn get_config(branch_name: Option<String>) -> Config {
    let branch = match branch_name {
        Some(v) => v,
        None => "main".to_string(),
    };

    Config::new(branch)
}

fn universe_id(config: Config) -> UniverseId {
    UniverseId(config.get_universe_id().unwrap())
}

impl DataStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            DataStoreCommands::ListStores {
                branch_name,
                prefix,
                limit,
                cursor,
                api_key,
            } => {
                let auth: String = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
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
                branch_name,
                prefix,
                limit,
                cursor,
                api_key,
                datastore_name,
                scope,
                all_scopes,
            } => {
                let auth: String = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);
                let config_datastore = config.get_datastore();

                let name = datastore_name.unwrap_or(config_datastore.0.unwrap());
                let scope = Some(scope.unwrap_or(config_datastore.1.unwrap()));

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
                let datastore = rbx_cloud.datastore();

                let mut has_cursor = true;
                let mut next_cursor = cursor.clone();

                while has_cursor {
                    let res = datastore
                        .list_entries(&DataStoreListEntries {
                            name: name.clone(),
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
                branch_name,
                datastore_name,
                scope,
                key,
                api_key,
            } => {
                let auth = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);
                let config_datastore = config.get_datastore();

                let name = datastore_name.unwrap_or(config_datastore.0.unwrap());
                let scope = Some(scope.unwrap_or(config_datastore.1.unwrap()));

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
                let datastore = rbx_cloud.datastore();
                let res = datastore
                    .get_entry_string(&DataStoreGetEntry { name, scope, key })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format_json(data))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Set {
                branch_name,
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
                let auth = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);
                let config_datastore = config.get_datastore();

                let name = datastore_name.unwrap_or(config_datastore.0.unwrap());
                let scope = Some(scope.unwrap_or(config_datastore.1.unwrap()));

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
                let datastore = rbx_cloud.datastore();
                let ids = u64_ids_to_roblox_ids(user_ids);
                let res = datastore
                    .set_entry(&DataStoreSetEntry {
                        name,
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
                branch_name,
                datastore_name,
                scope,
                key,
                increment_by,
                user_ids,
                attributes,
                api_key,
            } => {
                let auth = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);
                let config_datastore = config.get_datastore();

                let name = datastore_name.unwrap_or(config_datastore.0.unwrap());
                let scope = Some(scope.unwrap_or(config_datastore.1.unwrap()));

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
                let datastore = rbx_cloud.datastore();
                let ids = u64_ids_to_roblox_ids(user_ids);
                let res = datastore
                    .increment_entry(&DataStoreIncrementEntry {
                        name,
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
                branch_name,
                datastore_name,
                scope,
                key,
                api_key,
            } => {
                let auth = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);
                let config_datastore = config.get_datastore();

                let name = datastore_name.unwrap_or(config_datastore.0.unwrap());
                let scope = Some(scope.unwrap_or(config_datastore.1.unwrap()));

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
                let datastore = rbx_cloud.datastore();
                let res = datastore
                    .delete_entry(&DataStoreDeleteEntry { name, scope, key })
                    .await;
                match res {
                    Ok(_) => Ok(None),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::ListVersions {
                branch_name,
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
                let auth = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);
                let config_datastore = config.get_datastore();

                let name = datastore_name.unwrap_or(config_datastore.0.unwrap());
                let scope = Some(scope.unwrap_or(config_datastore.1.unwrap()));

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
                let datastore = rbx_cloud.datastore();

                let mut has_cursor = true;
                let mut next_cursor = cursor.clone();
                let order = sort_order.unwrap_or(ListEntrySortOrder::Ascending);

                while has_cursor {
                    let res = datastore
                        .list_entry_versions(&DataStoreListEntryVersions {
                            name: name.clone(),
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
                branch_name,
                datastore_name,
                scope,
                key,
                version_id,
                api_key,
            } => {
                let auth = getenv(api_key, "OPENCLOUD_KEY".to_string());
                let config = get_config(branch_name);
                let config_datastore = config.get_datastore();

                let name = datastore_name.unwrap_or(config_datastore.0.unwrap());
                let scope = Some(scope.unwrap_or(config_datastore.1.unwrap()));

                let rbx_cloud = RbxCloud::new(&auth, universe_id(config));
                let datastore = rbx_cloud.datastore();
                let res = datastore
                    .get_entry_version(&DataStoreGetEntryVersion {
                        name,
                        scope,
                        key,
                        version_id,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format_json(data))),
                    Err(err) => Err(err.into()),
                }
            }
        }
    }
}
