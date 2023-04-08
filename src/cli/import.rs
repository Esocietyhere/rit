use ansi_term::Colour;
use clap::Parser;
use std::process::Command;

/// Import assets, archives and/or maps
#[derive(Debug, Parser)]
pub struct ImportCommand {
    /// Whether to import assets
    #[clap(short, long, takes_value = false)]
    asset_flag: bool,
    /// Whether to import archives
    #[clap(short = 'r', long, takes_value = false)]
    archive_flag: bool,
    /// Whether to import all maps
    #[clap(short = 'M', long, takes_value = false)]
    map_flag: bool,
    /// The path to the place file
    #[clap(short, long, value_parser)]
    file_path: Option<String>,
    /// The name of the map to import
    #[clap(short, long, value_parser)]
    map_name: Option<String>,
}

fn remodel(command_name: &str, args: &[&str]) {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"remodel run remodel/scripts/{}.lua remodel {}"#,
            command_name,
            args.join(" ")
        ))
        .output()
        .expect("failed to execute process");
}

fn log(message: &str) {
    println!("{} {}", Colour::Green.paint("Importing"), message);
}

impl ImportCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        if self.asset_flag {
            remodel("import-assets", &[]);
            log("assets");
        }

        if self.archive_flag {
            remodel("import-archives", &[]);
            log("archives");
        }

        if self.map_flag {
            remodel("import-all-maps", &[]);
            log("all maps");
        }

        if self.map_name.is_some() {
            if self.file_path.is_some() {
                remodel(
                    "import-local-map",
                    &[
                        self.file_path.as_ref().unwrap(),
                        self.map_name.as_ref().unwrap(),
                    ],
                );
                log(&format!(
                    "local map \"{}\" from file \"{}\"",
                    self.map_name.as_ref().unwrap(),
                    self.file_path.as_ref().unwrap()
                ));
            } else {
                remodel("import-map", &[self.map_name.as_ref().unwrap()]);
                log(&format!("map \"{}\"", self.map_name.as_ref().unwrap()));
            }
        }

        Ok(None)
    }
}
