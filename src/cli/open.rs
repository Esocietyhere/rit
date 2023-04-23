use crate::color::Color;
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

fn open_output(file_path: String) -> String {
    format!("{} `{}`", Color::green().pad("Running"), file_path)
}

pub fn open_place(file_path: Option<String>) -> Option<String> {
    let input = file_path.unwrap_or(format!("build/{}.rbxl", "default"));
    let path = Path::new(&input);

    if path.exists() {
        println!("{}", open_output(input.clone()));
        opener::open(path).expect("Couldn't open Roblox Studio");
    } else {
        return Some(format!("File {:?} does not exist!", path));
    }

    None
}
