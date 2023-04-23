use super::{build::build_place, open::open_place};
use clap::Parser;

/// Build the project and open it in Roblox Studio
#[derive(Debug, Parser)]
pub struct RunCommand {
    /// The name of the project to build
    #[clap(short, long, value_parser)]
    project_name: Option<String>,
    /// The name of the output file
    #[clap(short, long, value_parser)]
    output_name: Option<String>,
}

impl RunCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let output = build_place(self.project_name.clone(), self.output_name.clone());
        open_place(output.clone());
        Ok(None)
    }
}
