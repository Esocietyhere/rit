mod datastore;
mod deploy;
mod devtools;
mod event;
mod studio;

pub use datastore::*;
pub use deploy::*;
pub use devtools::*;
pub use event::*;
pub use studio::*;

pub fn getenv(api_key: Option<String>, name: String) -> String {
    match api_key {
        Some(v) => v,
        None => std::env::var(name.clone())
            .unwrap_or_else(|_| panic!("environment variable \"{}\" is not set", name)),
    }
}
