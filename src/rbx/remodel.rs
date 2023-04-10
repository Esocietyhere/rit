use clap::Parser;
use regex::Regex;
use std::process::Command;

#[derive(Debug, Parser)]
pub struct Remodel {
    auth: String,
}

fn get_path(path: &str) -> String {
    format!("{}\\{}", env!("CARGO_MANIFEST_DIR"), path).replace('\\', "/")
}

fn get_command(file_name: &str, args: &[&str]) -> String {
    let remodel_path = get_path("remodel");
    let script_path = get_path(&format!("remodel\\scripts\\{}", file_name));

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
        .trim()
        .to_string()
}

impl Remodel {
    pub fn new(auth: String) -> Remodel {
        Remodel { auth }
    }

    pub fn run(&self, file_name: &str, args: &[&str]) {
        let remodel_command = format!("{} --auth \"{}\"", get_command(file_name, args), self.auth);
        Command::new("sh")
            .arg("-c")
            .arg(remodel_command)
            .output()
            .expect("failed to execute process");
    }
}
