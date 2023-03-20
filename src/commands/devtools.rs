use std::process::Command;

pub fn devtools() -> anyhow::Result<Option<String>> {
    // Trust roblox/tarmac
    Command::new("aftman")
        .arg("trust")
        .arg("roblox/tarmac")
        .output()
        .expect("Failed to execute command");

    // Add roblox/tarmac@latest
    Command::new("aftman")
        .arg("add")
        .arg("roblox/tarmac@latest")
        .output()
        .expect("Failed to execute command");

    // Trust rojo-rbx/remodel
    Command::new("aftman")
        .arg("trust")
        .arg("rojo-rbx/remodel")
        .output()
        .expect("Failed to execute command");

    // Add rojo-rbx/remodel@latest
    Command::new("aftman")
        .arg("add")
        .arg("rojo-rbx/remodel@latest")
        .output()
        .expect("Failed to execute command");

    // Trust rojo-rbx/rojo
    Command::new("aftman")
        .arg("trust")
        .arg("rojo-rbx/rojo")
        .output()
        .expect("Failed to execute command");

    // Add rojo-rbx/rojo@latest
    Command::new("aftman")
        .arg("add")
        .arg("rojo-rbx/rojo@latest")
        .output()
        .expect("Failed to execute command");

    // Trust upliftgames/wally
    Command::new("aftman")
        .arg("trust")
        .arg("upliftgames/wally")
        .output()
        .expect("Failed to execute command");

    // Add upliftgames/wally@latest
    Command::new("aftman")
        .arg("add")
        .arg("upliftgames/wally@latest")
        .output()
        .expect("Failed to execute command");

    // Trust kampfkarren/selene
    Command::new("aftman")
        .arg("trust")
        .arg("kampfkarren/selene")
        .output()
        .expect("Failed to execute command");

    // Add kampfkarren/selene@latest
    Command::new("aftman")
        .arg("add")
        .arg("kampfkarren/selene@latest")
        .output()
        .expect("Failed to execute command");

    // Trust johnnymorganz/stylua
    Command::new("aftman")
        .arg("trust")
        .arg("johnnymorganz/stylua")
        .output()
        .expect("Failed to execute command");

    // Add johnnymorganz/stylua@latest
    Command::new("aftman")
        .arg("add")
        .arg("johnnymorganz/stylua@latest")
        .output()
        .expect("Failed to execute command");

    Ok(None)
}
