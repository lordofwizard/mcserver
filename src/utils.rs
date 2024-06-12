// Collection of commonly used function I am using for mcserver

// Downloading a java version


use std::fs;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
pub fn make_file_tree(project_name: &str) {
    // This function takes a project_name i.e. Minecraft Server name and generates basic tree
    // structure if not present

    let base_path = Path::new(project_name);

    if !base_path.exists() {
        println!("Warning: The tree structure wasn't present. Generating a new project.");
        fs::create_dir(base_path).expect("Failed to create project directory");
    }

    let directories = vec!["cache", "java", "log", "server"];

    for dir in directories {
        let dir_path = base_path.join(dir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path).expect("Failed to create directory");
        }
    }
}

#[allow(dead_code)]
pub fn server_present() -> bool {
    let path = Path::new("config.toml");
    path.exists()
}



fn default_toml(project_name: &str) {
    if !Path::new(project_name).exists() {
        fs::create_dir_all(project_name).expect("Failed to create project directory");
    }

    let toml_content = r#"
[mcserver]
logfile = "mcserver.log"
tunnel = "playit"
java = "22"

[server]
online_mode = false
version = "1.20.0"
server_type = "java"
category = "vanilla"
providor = "vanilla"
#url = "https://launcher.mojang.com/v1/objects/9d3d8f07c1e2b3f3d3f6a3b5f7f2b0b3f7b2d4f2/server.jar"
"#;

    let file_path = format!("{}/config.toml", project_name);

    let mut file = fs::File::create(&file_path).expect("Failed to create TOML file");
    file.write_all(toml_content.as_bytes()).expect("Failed to write to TOML file");

    println!("TOML file created at {}", file_path);
}

