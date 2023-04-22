use clap::Parser;
use std::path::Path;

/// Open a place file in Roblox Studio
#[derive(Debug, Parser)]
pub struct OpenCommand {
    /// The path to the place file
    #[clap(short, long, value_parser)]
    file_path: Option<String>,
}

impl OpenCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let output = open_place(self.file_path.clone());
        Ok(output)
    }
}

pub fn open_place(file_path: Option<String>) -> Option<String> {
    let input = file_path.unwrap_or(format!("build/{}.rbxl", "default"));
    let path = Path::new(&input);

    if path.exists() {
        opener::open(path).expect("Couldn't open Roblox Studio");
    } else {
        Some(format!("File {:?} does not exist!", path));
    }

    None
}
