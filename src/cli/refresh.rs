use super::getenv;
use crate::rbx::Remodel;
use ansi_term::Colour;
use clap::Parser;

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

impl RefreshCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let auth = getenv(self.auth.clone(), "ROBLOSECURITY".to_string());
        let remodel = Remodel::new(auth);
        let project_name = self.project_name.clone().unwrap_or("default".to_string());
        remodel.run("refresh-project.lua", &[project_name.as_str()]);

        Ok(Some(format!(
            "{} {}",
            Colour::Green.paint("Refreshing"),
            project_name
        )))
    }
}
