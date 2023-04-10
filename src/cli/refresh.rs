use super::getenv;
use ansi_term::Colour;
use clap::Parser;
use regex::Regex;
use std::process::Command;

/// Refresh a project file
#[derive(Debug, Parser)]
pub struct RefreshCommand {
    /// The name of the project to refresh
    #[clap(short, long, value_parser)]
    project_name: Option<String>,
    /// The authentication token to use
    #[clap(short, long, value_parser)]
    auth: Option<String>,
}

fn get_path(path: &str) -> String {
    format!("{}\\{}", env!("CARGO_MANIFEST_DIR"), path).replace("\\", "/")
}

fn get_command(project_name: &str) -> String {
    let remodel_path = get_path("remodel");
    let script_path = get_path("remodel\\scripts\\refresh-project.lua");

    let command = format!(
        "remodel run {} {} {}",
        script_path, remodel_path, project_name
    );

    // Sanitized command
    Regex::new(r"\s+")
        .unwrap()
        .replace_all(&command, " ")
        .trim()
        .to_string()
}

struct Remodel {
    auth: String,
}

impl Remodel {
    pub fn new(auth: String) -> Remodel {
        Remodel { auth }
    }

    pub fn run(&self, project_name: &str) {
        let remodel_command = format!("{} --auth \"{}\"", get_command(project_name), self.auth);
        Command::new("sh")
            .arg("-c")
            .arg(remodel_command)
            .output()
            .expect("failed to execute process");

        println!("{} {}", Colour::Green.paint("Refreshing"), project_name);
    }
}

impl RefreshCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let auth = getenv(self.auth.clone(), "ROBLOSECURITY".to_string());
        let remodel = Remodel::new(auth);
        let project_name = self.project_name.clone().unwrap_or("default".to_string());
        remodel.run(&project_name);

        Ok(None)
    }
}
