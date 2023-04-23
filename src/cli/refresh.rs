use super::getenv;
use crate::color::Color;
use crate::rbx::Remodel;
use clap::Parser;
use fs_err as fs;
use regex::Regex;

/// Refresh a project file
#[derive(Debug, Parser)]
pub struct RefreshCommand {
    /// Whether to refresh all project files
    #[clap(short = 'A', long, takes_value = false)]
    all_projects: bool,
    /// The name of the project to refresh
    #[clap(short, long, value_parser)]
    project_name: Option<String>,
    /// The authentication token to use
    #[clap(short, long, value_parser)]
    auth: Option<String>,
}

fn filter_project_file(filename: &str) -> String {
    let pattern = Regex::new(r#"^.+[\\/]([^\\/]+)\.project\.json$"#).unwrap();

    if let Some(captures) = pattern.captures(filename) {
        captures.get(1).unwrap().as_str().to_string()
    } else {
        panic!("Invalid project file name: {}", filename)
    }
}

impl RefreshCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let auth = getenv(self.auth.clone(), "ROBLOSECURITY".to_string());
        let remodel = Remodel::new(auth);
        let project_name = self.project_name.clone().unwrap_or("default".to_string());

        if self.all_projects {
            for entry in fs::read_dir(".")? {
                let path = entry.unwrap().path();
                if let Some(extension) = path.extension() {
                    if extension == "json"
                        && path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .ends_with(".project.json")
                    {
                        let filtered_name = filter_project_file(path.to_string_lossy().as_ref());

                        if filtered_name != "default" {
                            remodel.run("refresh-project.lua", &[filtered_name.clone()]);
                            println!("{} {}", Color::green().pad("Refreshing"), filtered_name);
                        }
                    }
                }
            }
        } else if self.project_name.is_some() {
            remodel.run("refresh-project.lua", &[project_name.clone()]);
            println!("{} {}", Color::green().pad("Refreshing"), project_name);
        } else {
            println!("No project name specified!");
        }

        Ok(None)
    }
}
