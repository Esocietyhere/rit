use clap::Parser;
use regex::Regex;
use std::process::Command;

#[derive(Debug, Parser)]
pub struct Remodel {
    auth: String,
}

fn get_command(file_name: &str, args: &[String]) -> String {
    let script_path = format!("remodel/scripts/{}", file_name);
    let command = format!("remodel run {} remodel {}", script_path, args.join(" "));

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

    pub fn run(&self, file_name: &str, args: &[String]) {
        let remodel_command = format!("{} --auth \"{}\"", get_command(file_name, args), self.auth);
        Command::new("sh")
            .arg("-c")
            .arg(remodel_command)
            .output()
            .expect("failed to execute process");
    }
}
