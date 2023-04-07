use clap::Parser;
use std::process::Command;

/// Import assets
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
            r#"remodel run "remodel/scripts/{}.lua" remodel {}"#,
            command_name,
            args.join(" ")
        ))
        .output()
        .expect("failed to execute process");
}

impl ImportCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        if self.asset_flag {
            remodel("import-assets", &[]);
            println!("Importing assets");
        }

        if self.archive_flag {
            remodel("import-archives", &[]);
            println!("Importing archives");
        }

        if self.map_flag {
            remodel("import-all-maps", &[]);
            println!("Importing all maps");
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
                println!(
                    "Importing map {} from {}",
                    self.map_name.as_ref().unwrap(),
                    self.file_path.as_ref().unwrap()
                );
            } else {
                remodel("import-map", &[self.map_name.as_ref().unwrap()]);
                println!("Importing map {}", self.map_name.as_ref().unwrap());
            }
        }

        Ok(None)
    }
}
