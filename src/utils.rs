// Collection of commonly used function I am using for mcserver

// Downloading a java version 

pub fn download_jdk(java_version: u8) {

    #[derive(Deserialize)]
    struct AvailableReleases {
        available_releases: Vec<u8>,
    }
    use reqwest::blocking::get;
    use serde::Deserialize;
    use std::process::Command;
    // Make the initial API request
    let response = match get("https://api.adoptium.net/v3/info/available_releases") {
        Ok(res) => res,
        Err(_) => {
            println!("Failed to make the API request, to Adoptium");
            std::process::exit(1);
        }
    };

    // Parse the JSON response
    let releases: AvailableReleases = match response.json() {
        Ok(json) => json,
        Err(_) => {
            println!("Failed to parse the JSON response, from Adoptium");
            std::process::exit(1);
        }
    };

    // Check if the requested Java version is available
    if !releases.available_releases.contains(&java_version) {
        println!("Given Java Version : {} is not available by the sources. ", java_version);
        std::process::exit(0);
    }

    // Replace JAVA_VERSION in the URL
    let url = format!(
        "https://api.adoptium.net/v3/binary/latest/{}/ga/linux/x64/jdk/hotspot/normal/eclipse",
        java_version
    );

    // Use the CLI tool curl to download the file
    let output_file = format!("jdk-{}.tar.gz", java_version);
    let status = Command::new("curl")
        .arg("-L")
        .arg("--progress-bar")
        .arg(&url)
        .arg("-o")
        .arg(&output_file)
        .status()
        .expect("failed to execute curl command");

    if status.success() {
        println!("File downloaded successfully as {}", output_file);
    } else {
        println!("Failed to download the file.");
        std::process::exit(1);
    }
}
