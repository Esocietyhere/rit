use std::process::Command;
#[derive(Debug)]

pub struct BuildParams {
    pub project_name: String,
    pub output_name: String,
}

pub struct OpenPlaceParams {
    pub file_name: String,
}

pub struct SyncParams {
    pub auth: String,
}

pub fn init() -> anyhow::Result<Option<String>> {
    Command::new("sh")
        .arg("-c")
        .arg("rojo init && wally init && aftman init")
        .output()
        .expect("failed to execute process");
    Ok(None)
}

pub fn build(params: &BuildParams) -> anyhow::Result<Option<String>> {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"mkdir -p build && rojo --version && rojo build "{}.project.json" -o "build/{}.rbxl""#,
            params.project_name,
            params.output_name,
        ))
        .output()
        .expect("failed to execute process");

    println!("Built project {}!", params.project_name);
    Ok(None)
}

pub fn open_place(params: &OpenPlaceParams) -> anyhow::Result<Option<String>> {
    if cfg!(target_os = "windows") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                r#"start "build/{}.rbxl""#,
                params.file_name.clone()
            ))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "wsl") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                r#"start "build/{}.rbxl""#,
                params.file_name.clone()
            ))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                r#"xdg-open "build/{}.rbxl""#,
                params.file_name.clone()
            ))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"open "build/{}.rbxl""#, params.file_name.clone()))
            .output()
            .expect("failed to execute process");
    }
    Ok(None)
}

pub fn img_sync(params: &SyncParams) -> anyhow::Result<Option<String>> {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"tarmac sync --target roblox --auth "{}" --retry 3 --retry-delay 5"#,
            params.auth,
        ))
        .output()
        .expect("failed to execute process");
    Ok(None)
}
