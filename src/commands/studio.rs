use std::path::Path;
use std::process::Command;
use std::string::String;
use std::{env, fs};
#[derive(Debug)]

pub struct BuildParams {
    pub project_name: Option<String>,
    pub output_name: Option<String>,
}

pub struct OpenPlaceParams {
    pub file_path: Option<String>,
}

pub struct SyncParams {
    pub auth: Option<String>,
}

pub fn init() -> anyhow::Result<Option<String>> {
    Command::new("sh")
        .arg("-c")
        .arg("rojo init && wally init && aftman init")
        .output()
        .expect("failed to execute process");
    Ok(None)
}

pub fn build(params: &BuildParams) -> Option<String> {
    let project = params.project_name.clone().unwrap_or("default".to_string());
    let output = format!(
        "build/{}.rbxl",
        params.output_name.clone().unwrap_or(project.clone())
    );
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

pub fn open_place(params: &OpenPlaceParams) -> Option<String> {
    let input = params
        .file_path
        .clone()
        .unwrap_or(format!("build/{}.rbxl", "default"));
    let path = Path::new(&input);
    if !path.exists() {
        println!("File {} does not exist!", input);
        return None;
    };
    if cfg!(target_os = "windows") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"start "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "wsl") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"start "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"xdg-open "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"open "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else {
        println!("Unsupported operating system!");
    }
    Some(input)
}

pub fn img_sync(params: &SyncParams) -> anyhow::Result<Option<String>> {
    let auth = match params.auth.clone() {
        Some(v) => v,
        None => env::var("ROBLOSECURITY").expect("ROBLOSECURITY not set"),
    };

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"tarmac sync --target roblox --auth "{}" --retry 3 --retry-delay 5"#,
            auth,
        ))
        .output()
        .expect("failed to execute process");
    Ok(None)
}
