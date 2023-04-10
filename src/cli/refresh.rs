use super::getenv;
use ansi_term::Colour;
use clap::Parser;
use regex::Regex;
use std::{path::Path, process::Command};

/// Refresh a project file
#[derive(Debug, Parser)]
pub struct RefreshCommand {
    /// The path to the place file
    #[clap(short, long, value_parser)]
    file_path: Option<String>,
    /// The authentication token to use
    #[clap(short, long, value_parser)]
    auth: Option<String>,
}

fn get_path(path: &str) -> String {
    format!("{}\\{}", env!("CARGO_MANIFEST_DIR"), path).replace("\\", "/")
}

fn get_command(file_path: &str) -> String {
    let remodel_path = get_path("remodel");
    let script_path = get_path("remodel\\scripts\\refresh-project.lua");

    let command = format!("remodel run {} {} {}", script_path, remodel_path, file_path);

    // Sanitized command
    Regex::new(r"\s+")
        .unwrap()
        .replace_all(&command, " ")
        .to_string()
}

struct Remodel {
    auth: String,
}

impl Remodel {
    pub fn new(auth: String) -> Remodel {
        Remodel { auth }
    }

    pub fn run(&self, file_path: &str) {
        let remodel_command = format!("{}--auth \"{}\"", get_command(file_path), self.auth);
        Command::new("sh")
            .arg("-c")
            .arg(remodel_command)
            .output()
            .expect("failed to execute process");

        println!("{} {}", Colour::Green.paint("Refreshing"), file_path);
    }
}

impl RefreshCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let auth = getenv(self.auth.clone(), "ROBLOSECURITY".to_string());
        let remodel = Remodel::new(auth);

        let file_path = self
            .file_path
            .clone()
            .unwrap_or(format!("build/{}.rbxl", "default"));
        let path = Path::new(&file_path);

        if !path.exists() {
            panic!("File {} does not exist!", file_path);
        };

        remodel.run(&file_path);
        Ok(None)
    }
}
