use crate::{centro::fetch_latest_minecraft_version, java::latest_java_version};

pub fn config_builder(project_name: &str) {
    write_toml_file(project_name);
}

pub fn write_toml_file(project_name: &str) {
    // Call functions to get the latest versions
    let latest_java_version = latest_java_version();
    let latest_minecraft_version = fetch_latest_minecraft_version();

    // Define the template with placeholders
    let toml_template = r#"
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
"#;

    // Format the TOML string by replacing placeholders with actual values
    let toml_content = toml_template
        .replace("{project_name}", project_name)
        .replace("{latest_java_version}", &latest_java_version)
        .replace("{version}", &latest_minecraft_version);

    // Specify the path for the config.toml file
    let file_path = format!("./{}/config.toml", project_name);

    // Write the formatted TOML content to the file
    std::fs::create_dir_all(format!("./{}", project_name))
        .expect("Project Should Have been properly setuped"); // Ensure the directory exists
    std::fs::write(file_path, toml_content).expect("Project Should Have been properly setuped");
}

#[cfg(test)]
mod tests {}
