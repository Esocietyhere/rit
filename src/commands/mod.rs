mod datastore;
mod deploy;
mod devtools;
mod studio;

pub use datastore::*;
pub use deploy::*;
pub use devtools::*;
pub use studio::*;

pub fn getenv(api_key: Option<String>, name: String) -> String {
    let value = match api_key {
        Some(v) => v,
        None => std::env::var(name.clone()).expect(&format!(
            "environment variable \"{}\" is not set",
            name.clone()
        )),
    };
    return value;
}
