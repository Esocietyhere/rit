use clap::Parser;
use std::{io::Write, process::Command};
use tempfile::Builder;

static LIBRARY_TEMPLATE: &str = include_str!("lib.lua");

#[derive(Debug, Parser)]
pub struct Remodel {
    auth: String,
}

impl Remodel {
    pub fn new(auth: String) -> Remodel {
        Remodel { auth }
    }

    pub fn run(&self, method: &str, args: &[String]) {
        let complete_source = LIBRARY_TEMPLATE.replace("{{method}}", method).replace(
            "{{args}}",
            &args
                .iter()
                .map(|arg| format!("\"{}\"", arg))
                .collect::<Vec<_>>()
                .join(", "),
        );

        let mut temp_file = Builder::new()
            .prefix("rit-")
            .suffix(".lua")
            .tempfile()
            .unwrap();

        temp_file.write_all(complete_source.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let remodel_command = format!(
            "remodel run {} . --auth \"{}\"",
            temp_file.path().to_string_lossy().replace('\\', "/"),
            self.auth
        );
        Command::new("sh")
            .arg("-c")
            .arg(remodel_command)
            .output()
            .expect("failed to execute process");
    }
}
