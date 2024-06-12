// This file consist of functions related to generating the server itself
// Write functionalities to generate files regarding the toml file
// config.toml
//

use std::fs;
use std::path::Path;
use std::io::{self, Write};
use crate::utils::make_file_tree;
pub fn server_generate(){

    let mut input = String::new();

    // Prompt the user for input
    print!("Enter project name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trimmed_input = input.trim();
    let project_name: String = trimmed_input.replace(char::is_whitespace, "_");
    make_file_tree(&project_name);
    
    default_toml(&project_name);
    
    println!("Generating a new server hehe");
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

