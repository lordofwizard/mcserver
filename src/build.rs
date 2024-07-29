use crate::{
    config::Config,
    java::{download_jdk, file_exists},
};
use colored::*;

pub fn build() {
    let config: Config = Config::new_build();

    println!("Building a Minecraft server {}", config.project_name.blue());

    println!("Downloading the server");

    if file_exists("./server/server.jar") {
        println!(
            "{}, {}!",
            "File already exists".green(),
            "Aborting".yellow()
        );
        std::process::exit(0);
    } else {
        let url = format!(
            "https://centrojars.com/api/fetchJar/{}/{}/{}/",
            config.category, config.providor, config.version
        );

        let status = std::process::Command::new("curl")
            .arg("-X")
            .arg("GET")
            .arg("-L")
            .arg("--progress-bar")
            .arg(&url)
            .arg("-o")
            .arg("./server/server.jar")
            .status()
            .expect("failed to execute curl command");

        if status.success() {
            println!("File downloaded successfully as {}", "./server/server.jar");
        } else {
            println!("Failed to download the file.");
            std::process::exit(1);
        }
    }
}
