use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error as IoError;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    mcserver: Option<ConfigTomlMcServer>,
    server: Option<ConfigTomlServer>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlMcServer {
    project_name: Option<String>,
    logfile: Option<String>,
    tunnel: Option<String>,
    java: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlServer {
    online_mode: Option<bool>,
    version: Option<String>,
    server_type: Option<String>,
    category: Option<String>,
    providor: Option<String>,
    url: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub project_name: String,
    pub logfile: String,
    pub tunnel: String,
    pub java: String,
    pub online_mode: bool,
    pub version: String,
    pub server_type: String,
    pub category: String,
    pub providor: String,
    pub url: String,
}



impl Config {
    pub fn new(project_name: &str) -> Self {
        let f1 = format!("./{}/config.toml", project_name);
        let f2 = format!("./{}/config.toml", project_name);

        let config_filepaths: [&str; 2] = [&f1, &f2];
        println!("{:?}", config_filepaths);
        let mut content: String = "".to_owned();

        for filepath in &config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml {
                mcserver: None,
                server: None,
            }
        });

        let (project_name, logfile, tunnel, java): (String, String, String, String) =
            match config_toml.mcserver {
                Some(mcserver) => {
                    let mc_project_name: String = mcserver.project_name.unwrap_or_else(|| {
                        println!("Missing field project_name in table mcserver.");
                        "unknown".to_owned()
                    });
                    let mc_logfile: String = mcserver.logfile.unwrap_or_else(|| {
                        println!("Missing field logfile in table mcserver.");
                        "unknown".to_owned()
                    });

                    let mc_tunnel: String = mcserver.tunnel.unwrap_or_else(|| {
                        println!("Missing field tunnel in table mcserver.");
                        "unknown".to_owned()
                    });

                    let mc_java: String = mcserver.java.unwrap_or_else(|| {
                        println!("Missing field java in table mcserver.");
                        "unknown".to_owned()
                    });
                    (mc_project_name, mc_logfile, mc_tunnel, mc_java)
                }
                None => {
                    println!("Missing table mcserver.");
                    (
                        "unknown".to_owned(),
                        "unknown".to_owned(),
                        "unknown".to_owned(),
                        "unknown".to_owned(),
                    )
                }
            };

        let (online_mode, version, server_type, category, providor, url): (
            bool,
            String,
            String,
            String,
            String,
            String,
        ) = match config_toml.server {
            Some(server) => {
                let srv_online_mode: bool = server.online_mode.unwrap_or_else(|| {
                    println!("Missing field online_mode in table server.");
                    false
                });

                let srv_version: String = server.version.unwrap_or_else(|| {
                    println!("Missing field version in table server.");
                    "unknown".to_owned()
                });

                let srv_type: String = server.server_type.unwrap_or_else(|| {
                    println!("Missing field type in table server.");
                    "unknown".to_owned()
                });
                let srv_category: String = server.category.unwrap_or_else(|| {
                    println!("Missing field type in table server.");
                    "unknown".to_owned()
                });
                let srv_providor: String = server.providor.unwrap_or_else(|| {
                    println!("Missing field type in table server.");
                    "unknown".to_owned()
                });

                let srv_url: String = server.url.unwrap_or_else(|| {
                    println!("Missing field url in table server.");
                    "unknown".to_owned()
                });
                (
                    srv_online_mode,
                    srv_version,
                    srv_type,
                    srv_category,
                    srv_providor,
                    srv_url,
                )
            }
            None => {
                println!("Missing table server.");
                (
                    false,
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                )
            }
        };

        Config {
            project_name,
            logfile,
            tunnel,
            java,
            online_mode,
            version,
            server_type,
            category,
            providor,
            url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    // Helper function to create a project directory with a config.toml file inside
    fn create_project_with_toml(project_name: &str, content: &str) -> TempDir {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let project_path = tmp_dir.path().join(project_name);

        // Create project directory
        fs::create_dir_all(&project_path).expect("Failed to create project directory");

        // Create the config.toml file
        let file_path = project_path.join("config.toml");
        let mut file = File::create(&file_path).expect("Failed to create config.toml file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to config.toml file");

        tmp_dir
    }

    #[test]
    fn test_valid_toml() {
        let toml_content = r#"
            [mcserver]
            project_name = "TestProject"
            logfile = "mcserver.log"
            tunnel = "playit"
            java = "23"

            [server]
            online_mode = true
            version = "1.21"
            server_type = "java"
            category = "vanilla"
            providor = "vanilla"
            url = "https://example.com"
        "#;

        // Create a temporary project directory with config.toml inside
        let project_name = "my_project";
        let tmp_dir = create_project_with_toml(project_name, toml_content);

        // Test the Config::new method
        let config = Config::new(project_name);

        assert_eq!(config.project_name, "TestProject");
        assert_eq!(config.logfile, "mcserver.log");
        assert_eq!(config.tunnel, "playit");
        assert_eq!(config.java, "23");

        assert_eq!(config.online_mode, true);
        assert_eq!(config.version, "1.21");
        assert_eq!(config.server_type, "java");
        assert_eq!(config.category, "vanilla");
        assert_eq!(config.providor, "vanilla");
        assert_eq!(config.url, "https://example.com");
    }

    #[test]
    fn test_missing_mcserver_fields() {
        let toml_content = r#"
            [mcserver]
            project_name = "TestProject"

            [server]
            online_mode = true
            version = "1.21"
            server_type = "java"
            category = "vanilla"
            providor = "vanilla"
            url = "https://example.com"
        "#;

        // Create a temporary project directory with config.toml inside
        let project_name = "my_project_missing_mcserver";
        let tmp_dir = create_project_with_toml(project_name, toml_content);

        let config = Config::new(project_name);

        assert_eq!(config.project_name, "TestProject");
        assert_eq!(config.logfile, "unknown");  // Missing field
        assert_eq!(config.tunnel, "unknown");   // Missing field
        assert_eq!(config.java, "unknown");     // Missing field
    }

    #[test]
    fn test_missing_server_fields() {
        let toml_content = r#"
            [mcserver]
            project_name = "TestProject"
            logfile = "mcserver.log"
            tunnel = "playit"
            java = "23"

            [server]
            online_mode = true
        "#;

        // Create a temporary project directory with config.toml inside
        let project_name = "my_project_missing_server";
        let tmp_dir = create_project_with_toml(project_name, toml_content);

        let config = Config::new(project_name);

        assert_eq!(config.online_mode, true);
        assert_eq!(config.version, "unknown");  // Missing field
        assert_eq!(config.server_type, "unknown");  // Missing field
        assert_eq!(config.category, "unknown");  // Missing field
        assert_eq!(config.providor, "unknown");  // Missing field
        assert_eq!(config.url, "unknown");  // Missing field
    }

    #[test]
    fn test_missing_both_sections() {
        let toml_content = r#""#;  // Empty content

        // Create a temporary project directory with empty config.toml inside
        let project_name = "my_project_missing_both";
        let tmp_dir = create_project_with_toml(project_name, toml_content);

        let config = Config::new(project_name);

        assert_eq!(config.project_name, "unknown");  // Missing section
        assert_eq!(config.online_mode, false);       // Missing section
    }
}
