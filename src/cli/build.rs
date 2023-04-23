use crate::color::Color;
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
        build_place(self.project_name.clone(), self.output_name.clone());
        Ok(None)
    }
}

fn build_output(project: String, output_path: String) -> String {
    format!(
        "{} {} ({})",
        Color::green().pad("Building"),
        project,
        output_path
    )
}

pub fn build_place(project_name: Option<String>, output_name: Option<String>) -> Option<String> {
    let project = project_name.unwrap_or("default".to_string());
    let output = format!("build/{}.rbxl", output_name.unwrap_or(project.clone()));
    let path = Path::new(&output).parent().unwrap();

    if !path.exists() {
        fs::create_dir_all(path).expect("failed to create directory");
    };

    println!("{}", build_output(project.clone(), output.clone()));
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
