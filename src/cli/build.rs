use clap::Parser;
use std::path::Path;
use std::process::Command;

use fs_err as fs;

/// Build the rojo project
#[derive(Debug, Parser)]
pub struct BuildCommand {
    /// The name of the project to build
    #[clap(short, long, value_parser)]
    project_name: Option<String>,
    /// The name of the output file
    #[clap(short, long, value_parser)]
    output_name: Option<String>,
}

impl BuildCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let output = build(self.project_name.clone(), self.output_name.clone());
        Ok(output)
    }
}

pub fn build(project_name: Option<String>, output_name: Option<String>) -> Option<String> {
    let project = project_name.unwrap_or("default".to_string());
    let output = format!("build/{}.rbxl", output_name.unwrap_or(project.clone()));
    let path = Path::new(&output).parent().unwrap();

    if !path.exists() {
        fs::create_dir_all(path).expect("failed to create directory");
    };

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"rojo --version && rojo build "{}.project.json" -o "{}""#,
            project, output,
        ))
        .output()
        .expect("failed to execute process");
    Some(output)
}
