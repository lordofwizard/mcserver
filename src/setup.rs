// This file consist of functions related to generating the server itself
// Write functionalities to generate files regarding the toml file
// config.toml
//

use crate::java::latest_java_version;

use crate::utils::make_file_tree;
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::config::*;

pub fn setup() {
    let project_name: String = project();

    project_exists_check(&project_name);

    make_file_tree(&project_name);

    default_toml(&project_name);

    let config: Config = Config::new(project_name.as_str());

    println!(
        "{} {}{}",
        "Project created with".green(),
        config.project_name.blue(),
        "!".green()
    );

    println!(
        "To build the server, do {}\nand run  {}",
        format!("cd {}", config.project_name).green(),
        "mcserver build".green()
    );
}

fn project() -> String {
    let mut input = String::new();
    print!("Enter project name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed_input = input.trim();
    trimmed_input.replace(char::is_whitespace, "_")
}

fn default_toml(project_name: &str) {
    if !Path::new(project_name).exists() {
        fs::create_dir_all(project_name).expect("Failed to create project directory");
    }

    let version = fetch_version();

    let toml_content: &str = &format!(
        r#"
[mcserver]
project_name = "{project_name}"
logfile = "mcserver.log"
tunnel = "playit"
java = "{latest_java_version}"

[server]
online_mode = false
version = "{version}"
server_type = "java"
category = "vanilla"
providor = "vanilla"
"#,
        latest_java_version = latest_java_version(),
        version = version
    );
    let file_path: String = format!("{}/config.toml", project_name);

    let mut file = fs::File::create(&file_path).expect("Failed to create TOML file");
    file.write_all(toml_content.as_bytes())
        .expect("Failed to write to TOML file");

    // println!("TOML file created at {}", file_path);
}

fn project_exists_check(project_name: &str) {
    let base_path = Path::new(project_name);

    if base_path.exists() {
        println!(
            "{}",
            "Project Already Found at the present location. Aborting".yellow()
        );
        std::process::exit(1);
    }
}

use reqwest::blocking::get;
use serde::Deserialize;

fn fetch_version() -> String {
    let url = "https://centrojars.com/api/fetchJar/vanilla/vanilla/";

    #[derive(Deserialize)]
    struct ApiResponse {
        status: String,
        response: Option<ResponseDetails>,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct ResponseDetails {
        version: String,
        file: String,
        size: Size,
        md5: String,
        built: u64,
        stability: String,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Size {
        display: String,
        bytes: u64,
    }

    let response = match get(url) {
        Ok(resp) => match resp.json::<ApiResponse>() {
            Ok(json) => json,
            Err(_) => return "unknown".to_string(),
        },
        Err(_) => return "unknown".to_string(),
    };

    if response.status == "success" {
        if let Some(details) = response.response {
            return details.version;
        }
    }

    "unknown".to_string()
}
