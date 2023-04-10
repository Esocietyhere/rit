use super::getenv;
use ansi_term::Colour;
use clap::Parser;
use regex::Regex;
use std::process::Command;

/// Import assets and maps
#[derive(Debug, Parser)]
pub struct ImportCommand {
    /// Whether to import assets
    #[clap(short, long, takes_value = false)]
    asset_flag: bool,
    /// Whether to import all maps
    #[clap(short = 'M', long, takes_value = false)]
    map_flag: bool,
    /// The path to the place file
    #[clap(short, long, value_parser)]
    file_path: Option<String>,
    /// The name of the map to import
    #[clap(short, long, value_parser)]
    map_name: Option<String>,
    /// The authentication token to use
    #[clap(short = 'A', long, value_parser)]
    auth: Option<String>,
}

fn get_path(path: &str) -> String {
    format!("{}\\{}", env!("CARGO_MANIFEST_DIR"), path).replace("\\", "/")
}

fn get_command(import_name: &str, args: &[&str]) -> String {
    let remodel_path = get_path("remodel");
    let script_path = get_path(&format!("remodel\\scripts\\import-{}.lua", import_name));

    println!("Script path: {}", script_path);

    let command = format!(
        "remodel run {} {} {}",
        script_path,
        remodel_path,
        args.join(" ")
    );

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

    pub fn run(&self, import_name: &str, args: &[&str]) {
        let remodel_command = format!("{}--auth \"{}\"", get_command(import_name, args), self.auth);
        Command::new("sh")
            .arg("-c")
            .arg(remodel_command)
            .output()
            .expect("failed to execute process");

        println!("{} {}", Colour::Green.paint("Importing"), import_name);
    }
}

impl ImportCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let auth = getenv(self.auth.clone(), "ROBLOSECURITY".to_string());
        let remodel = Remodel::new(auth);

        if self.asset_flag {
            remodel.run("assets", &[]);
        }

        if self.map_flag {
            remodel.run("all-maps", &[]);
        }

        if self.map_name.is_some() {
            if self.file_path.is_some() {
                remodel.run(
                    "local-map",
                    &[
                        self.file_path.as_ref().unwrap(),
                        self.map_name.as_ref().unwrap(),
                    ],
                );
            } else {
                remodel.run("map", &[self.map_name.as_ref().unwrap()]);
            }
        }

        Ok(None)
    }
}
